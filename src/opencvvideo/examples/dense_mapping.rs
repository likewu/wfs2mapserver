#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::fmt::format;
use std::path::Path;
use std::str::{self, FromStr};
use std::fs::{File};
use std::io::{BufReader, BufRead};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, prelude::*, core::Vector, core::KeyPoint, Result};
use nalgebra::{Matrix2, Point2, Vector2, Vector3, Isometry3, Translation3, UnitQuaternion, Quaternion};

use rand::prelude::*;

/**********************************************
* 本程序演示了单目相机在已知轨迹下的稠密深度估计
* 使用极线搜索 + NCC 匹配的方式，与书本的 12.2 节对应
* 请注意本程序并不完美，你完全可以改进它——我其实在故意暴露一些问题(这是借口)。
***********************************************/

// ------------------------------------------------------------------
// parameters
const boarder:i32 = 20;         // 边缘宽度
const width:i32 = 640;          // 图像宽度
const height:i32 = 480;         // 图像高度
const fx:f64 = 481.2;       // 相机内参
const fy:f64 = -480.0;
const cx:f64 = 319.5;
const cy:f64 = 239.5;
const ncc_window_size:i32 = 3;    // NCC 取的窗口半宽度
const ncc_area:i32 = (2 * ncc_window_size + 1) * (2 * ncc_window_size + 1); // NCC窗口面积
const min_cov:f64 = 0.1;     // 收敛判定：最小方差
const max_cov:f64 = 10.0;      // 发散判定：最大方差

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  //let window = "ORB features";
  //highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

  // 从数据集读取数据
  let mut color_image_files:Vec<String>=vec![];
  let mut poses_TWC:Vec<Isometry3<f64>>=vec![];
  let mut ref_depth=Mat::new_rows_cols_with_default(height, width, core::CV_64F, core::Scalar::all(1.)).unwrap();
  let ret = readDatasetFiles("E:\\app\\julia\\wfs2map\\src\\opencvvideo\\tests\\test_data", &mut color_image_files, &mut poses_TWC, &mut ref_depth);
  println!("read total {} files.", color_image_files.len());

  // 第一张图
  let ref1 = imgcodecs::imread(&color_image_files[0], imgcodecs::IMREAD_GRAYSCALE)?;
  let pose_ref_TWC = poses_TWC[0];
  let init_depth = 3.0;    // 深度初始值
  let init_cov2 = 3.0;     // 方差初始值
  let mut depth=Mat::new_rows_cols_with_default(height, width, core::CV_64F, core::Scalar::new(init_depth,init_depth,init_depth,init_depth)).unwrap();             // 深度图
  let mut depth_cov2=Mat::new_rows_cols_with_default(height, width, core::CV_64F, core::Scalar::new(init_cov2,init_cov2,init_cov2,init_cov2)).unwrap();         // 深度图方差

  for index in 1..color_image_files.len() {
      println!("*** loop {} ***", index);
      let curr = imgcodecs::imread(&color_image_files[index], imgcodecs::IMREAD_GRAYSCALE)?;
      if curr.data().is_null() {continue;}
      let pose_curr_TWC = poses_TWC[index];
      let pose_T_C_R = pose_curr_TWC.inverse() * pose_ref_TWC;   // 坐标转换关系： T_C_W * T_W_R = T_C_R
      update(&ref1, &curr, &pose_T_C_R, &mut depth, &mut depth_cov2);
      evaludateDepth(&ref_depth, &depth);
      plotDepth(&ref_depth, &depth);
      highgui::imshow("image", &curr);
      highgui::wait_key(1);
  }

  println!("estimation returns, saving depth map ...");
  imgcodecs::imwrite_def("depth.png", &depth);
  println!("done.");

  Ok(())
}

fn readDatasetFiles(
    path:&str,
    color_image_files:&mut Vec<String>,
    poses:&mut Vec<Isometry3<f64>>,
    ref_depth:&mut Mat) -> bool {
    let input = File::open(String::from(path) + &String::from("/first_200_frames_traj_over_table_input_sequence.txt")).unwrap();
    let mut fin = BufReader::new(input);

    let mut input = String::new();
    while 0!=fin.read_line(&mut input).unwrap() {
        // 数据格式：图像文件名 tx, ty, tz, qx, qy, qz, qw ，注意是 TWC 而非 TCW
        let mut words = input.split_whitespace();
        let cnt=words.clone().count();
        if cnt>8 {
            println!("words err:{} ",input);
        }
        let image=words.next().unwrap();
        let mut data=["";7]; 
        for i in 0..cnt-1 {
          data[i]=words.next().unwrap();
        }

        color_image_files.push(String::from(path)+"/images/"+image);
        poses.push(
            Isometry3::from_parts(Translation3::new(f64::from_str(data[0]).unwrap(), f64::from_str(data[1]).unwrap(), f64::from_str(data[2]).unwrap()),
                UnitQuaternion::from_quaternion(Quaternion::new(f64::from_str(data[6]).unwrap(), f64::from_str(data[3]).unwrap(), f64::from_str(data[4]).unwrap(), f64::from_str(data[5]).unwrap())) )
        );
        input.clear();
    }

    // load reference depth
    let input = File::open(String::from(path) + &String::from("/depthmaps/scene_000.depth")).unwrap();
    let mut fin = BufReader::new(input);
    for y in 0..height {
        for x in 0..width {
            //let depth = 0;
            let mut depth = vec![];
            fin.read_until(b' ', &mut depth);
            let depth=f64::from_str(str::from_utf8(&depth).unwrap().trim()).unwrap();
            *ref_depth.at_2d_mut::<f64>(y,x).unwrap() = depth / 100.0;
        }
    }
    println!("last ref_depth:{} ", ref_depth.at_2d::<f64>(height-1,width-1).unwrap());

    return true;
}

// 对整个深度图进行更新
fn update(ref1:&Mat, curr:&Mat, T_C_R:&Isometry3<f64>, depth:&mut Mat, depth_cov2:&mut Mat) {
    for x in boarder..(width - boarder) {
        for y in boarder..(height - boarder) {
            // 遍历每个像素
            //println!("depth_cov2.at_2d::<f64>(y,x): {}", depth_cov2.at_2d::<f64>(y,x).unwrap());
            if depth_cov2.at_2d::<f64>(y,x).unwrap() < &min_cov || depth_cov2.at_2d::<f64>(y,x).unwrap() > &max_cov // 深度已收敛或发散
                {continue;}
            // 在极线上搜索 (x,y) 的匹配
            let (pt_curr, epipolar_direction) = epipolarSearch(
                ref1,
                curr,
                T_C_R,
                &Vector2::<f64>::new(x as f64, y as f64),
                &depth.at_2d::<f64>(y,x).unwrap(),
                &depth_cov2.at_2d::<f64>(y,x).unwrap().sqrt()
            );
            if let None=pt_curr // 匹配失败
                {continue;}
            let pt_curr=pt_curr.unwrap();
            let epipolar_direction=epipolar_direction.unwrap();
                
            // 取消该注释以显示匹配
            // showEpipolarMatch(ref, curr, Vector2d(x, y), pt_curr);

            // 匹配成功，更新深度图
            updateDepthFilter(&Vector2::<f64>::new(x as f64, y as f64), &pt_curr, T_C_R, &epipolar_direction, depth, depth_cov2);
        }
    }
}

// 极线搜索
// 方法见书 12.2 12.3 两节
fn epipolarSearch(
    ref1:&Mat, curr:&Mat,
    T_C_R:&Isometry3<f64>, pt_ref:&Vector2<f64>,
    depth_mu:&f64, depth_cov:&f64) -> (Option<Vector2<f64>>, Option<Vector2<f64>>) {
    let mut f_ref = px2cam(pt_ref);
    //let f_ref = f_ref.try_normalize(0.0).unwrap();
    let f_ref = f_ref.normalize();
    let P_ref = f_ref.map(|e|{e*depth_mu});    // 参考帧的 P 向量

    let px_mean_curr = cam2px(&(T_C_R * P_ref)); // 按深度均值投影的像素
    let mut d_min = depth_mu - 3.0 * depth_cov;
    let d_max = depth_mu + 3.0 * depth_cov;
    if d_min<0.1 {d_min = 0.1;}
    let px_min_curr = cam2px(&(T_C_R * (f_ref * d_min)));    // 按最小深度投影的像素
    let px_max_curr = cam2px(&(T_C_R * (f_ref * d_max)));    // 按最大深度投影的像素
    
    //println!("px_mean_curr:{} px_min_curr:{} px_max_curr:{} ", px_mean_curr, px_min_curr, px_max_curr);

    let mut epipolar_line:Vector2<f64> = px_max_curr - px_min_curr;    // 极线（线段形式）
    let epipolar_direction:Vector2<f64> = match epipolar_line.try_normalize(0.0) {
        None => Vector2::<f64>::new(0.0,0.0),
        Some(e) => e,
    };
    let mut half_length = 0.5 * epipolar_line.norm();    // 极线线段的半长度
    if half_length>100.0 {half_length = 100.0;}   // 我们不希望搜索太多东西

    //println!("epipolar_line.norm:{} {} {}", epipolar_line.norm(), epipolar_line, Vector2::<f64>::new(0.0,0.0).normalize().norm());

    // 取消此句注释以显示极线（线段）
    // showEpipolarLine( ref, curr, pt_ref, px_min_curr, px_max_curr );

    // 在极线上搜索，以深度均值点为中心，左右各取半长度
    let mut best_ncc = -1.0;
    let mut best_px_curr=None;
    let mut l=-half_length;
    while l<=half_length { // l+=sqrt(2)
        let px_curr:Vector2<f64> = px_mean_curr + epipolar_direction.map(|e|{l*e});  // 待匹配点
        l += 0.7;
        if !inside(&px_curr) {
            continue;
        }
        // 计算待匹配点与参考帧的 NCC
        let ncc = NCC(ref1, curr, pt_ref, &px_curr);
        if ncc>best_ncc {
            best_ncc = ncc;
            best_px_curr = Some(px_curr);
        }
    }
    if best_ncc<0.85_f64 {      // 只相信 NCC 很高的匹配
        return (None, None);
    }
    (best_px_curr, Some(epipolar_direction))
}

fn NCC(ref1:&Mat, curr:&Mat,
    pt_ref:&Vector2<f64>, pt_curr:&Vector2<f64>) ->f64 {
    // 零均值-归一化互相关
    // 先算均值
    let mut mean_ref:f64 = 0.0;
    let mut mean_curr:f64 = 0.0;
    let mut values_ref:Vec<f64>=vec![];
    let mut values_curr:Vec<f64>=vec![]; // 参考帧和当前帧的均值
    for x in -ncc_window_size..=ncc_window_size {
        for y in -ncc_window_size..=ncc_window_size {
            let value_ref:f64 = ref1.at_2d::<f64>(y + pt_ref[1] as i32,x + pt_ref[0] as i32).unwrap() / 255.0;
            mean_ref += value_ref;

            let value_curr = getBilinearInterpolatedValue(curr, pt_curr + &Vector2::<f64>::new(x as f64, y as f64));
            mean_curr += value_curr;

            values_ref.push(value_ref);
            values_curr.push(value_curr);
        }
    }

    mean_ref /= ncc_area as f64;
    mean_curr /= ncc_area as f64;

    // 计算 Zero mean NCC
    let mut numerator:f64 = 0.0;
    let mut demoniator1:f64 = 0.0;
    let mut demoniator2:f64 = 0.0;
    for  i in 0..values_ref.len() {
        let n:f64 = (values_ref[i] - mean_ref) * (values_curr[i] - mean_curr);
        numerator += n;
        demoniator1 += (values_ref[i] - mean_ref) * (values_ref[i] - mean_ref);
        demoniator2 += (values_curr[i] - mean_curr) * (values_curr[i] - mean_curr);
    }
    return numerator / (demoniator1 * demoniator2 + 1.0E-10).sqrt();   // 防止分母出现零
}

fn updateDepthFilter(
    pt_ref:&Vector2<f64>,
    pt_curr:&Vector2<f64>,
    T_C_R:&Isometry3<f64>,
    epipolar_direction:&Vector2<f64>,
    depth:&mut Mat,
    depth_cov2:&mut Mat) -> bool {
    // 不知道这段还有没有人看
    // 用三角化计算深度
    let T_R_C = T_C_R.inverse();
    let f_ref = px2cam(pt_ref);
    let f_ref = f_ref.normalize();
    let f_curr = px2cam(pt_curr);
    let f_curr = f_curr.normalize();

    // 方程
    // d_ref * f_ref = d_cur * ( R_RC * f_cur ) + t_RC
    // f2 = R_RC * f_cur
    // 转化成下面这个矩阵方程组
    // => [ f_ref^T f_ref, -f_ref^T f2 ] [d_ref]   [f_ref^T t]
    //    [ f_2^T f_ref, -f2^T f2      ] [d_cur] = [f2^T t   ]
    let t = T_R_C.translation.vector;
    let f2:Vector3<f64> = T_R_C.rotation.to_rotation_matrix() * f_curr;
    let b:Vector2<f64> = Vector2::<f64>::new(t.dot(&f_ref), t.dot(&f2));
    let mut A:Matrix2<f64>=Default::default();
    A[(0, 0)] = f_ref.dot(&f_ref);
    A[(0, 1)] = -f_ref.dot(&f2);
    A[(1, 0)] = -A[(0, 1)];
    A[(1, 1)] = -f2.dot(&f2);
    let ans:Vector2<f64> = A.try_inverse().unwrap() * b;
    let xm:Vector3<f64> = ans[0] * f_ref;           // ref 侧的结果
    let xn:Vector3<f64> = t + ans[1] * f2;          // cur 结果
    let p_esti:Vector3<f64> = (xm + xn) / 2.0;      // P的位置，取两者的平均
    let depth_estimation = p_esti.norm();   // 深度值

    // 计算不确定性（以一个像素为误差）
    let p:Vector3<f64> = f_ref * depth_estimation;
    let a:Vector3<f64> = p - t;
    let t_norm = t.norm();
    let a_norm = a.norm();
    let alpha = (f_ref.dot(&t) / t_norm).acos();
    let beta = (-a.dot(&t) / (a_norm * t_norm)).acos();
    let f_curr_prime = px2cam(&(pt_curr + epipolar_direction));
    let f_curr_prime=f_curr_prime.normalize();
    let beta_prime = (f_curr_prime.dot(&-t) / t_norm).acos();
    let gamma = std::f64::consts::PI - alpha - beta_prime;
    let p_prime = t_norm * beta_prime.sin() / gamma.sin();
    let d_cov = p_prime - depth_estimation;
    let d_cov2 = d_cov * d_cov;

    // 高斯融合
    let mu = depth.at_2d::<f64>(pt_ref[1] as i32,pt_ref[0] as i32).unwrap();
    let sigma2 = depth_cov2.at_2d::<f64>(pt_ref[1] as i32,pt_ref[0] as i32).unwrap();

    let mu_fuse = (d_cov2 * mu + sigma2 * depth_estimation) / (sigma2 + d_cov2);
    let sigma_fuse2 = (sigma2 * d_cov2) / (sigma2 + d_cov2);

    *depth.at_2d_mut::<f64>(pt_ref[1] as i32,pt_ref[0] as i32).unwrap() = mu_fuse;
    *depth_cov2.at_2d_mut::<f64>(pt_ref[1] as i32,pt_ref[0] as i32).unwrap() = sigma_fuse2;

    true
}

// 后面这些太简单我就不注释了（其实是因为懒）
fn plotDepth(depth_truth:&Mat, depth_estimate:&Mat) {
    highgui::imshow("depth_truth", &(depth_truth * 0.4).into_result().unwrap().to_mat().unwrap());
    highgui::imshow("depth_estimate", &(depth_estimate * 0.4).into_result().unwrap().to_mat().unwrap());
    highgui::imshow("depth_error", &(depth_truth - depth_estimate).into_result().unwrap().to_mat().unwrap());
    let _ = highgui::wait_key(1);
}

fn evaludateDepth(depth_truth:&Mat, depth_estimate:&Mat) {
    let mut ave_depth_error = 0.0;     // 平均误差
    let mut ave_depth_error_sq = 0.0;      // 平方误差
    let mut cnt_depth_data = 0;
    for y in boarder..(depth_truth.rows() - boarder) {
        for x in boarder..(depth_truth.cols() - boarder) {
            let error = depth_truth.at_2d::<f64>(y as i32,x as i32).unwrap() - depth_estimate.at_2d::<f64>(y as i32,x as i32).unwrap();
            ave_depth_error += error;
            ave_depth_error_sq += error * error;
            cnt_depth_data+=1;
        }
    }
    ave_depth_error /= cnt_depth_data as f64;
    ave_depth_error_sq /= cnt_depth_data as f64;

    println!("Average squared error = {}, average error: {}", ave_depth_error_sq, ave_depth_error);
}

// 双线性灰度插值
#[inline]
fn getBilinearInterpolatedValue(img:&Mat, pt:Vector2::<f64>) -> f64 {
    let d = unsafe {img.data().offset((pt[1] as isize) * img.mat_step().get(0) as isize + (pt[0] as isize))};
    let xx:f64 = pt[0] - pt[0].floor();
    let yy:f64 = pt[1] - pt[1].floor();
    return ((1.0 - xx) * (1.0 - yy) * unsafe {*d.offset(0) as f64} +
            xx * (1.0 - yy) * unsafe {*d.offset(1) as f64} +
            (1.0 - xx) * yy * unsafe {*d.offset(img.mat_step().get(0) as isize) as f64} +
            xx * yy * unsafe {*d.offset(img.mat_step().get(0) as isize + 1) as f64}) / 255.0;
}

// 像素到相机坐标系
#[inline]
fn px2cam(px:&Vector2<f64>) -> Vector3<f64> {
    Vector3::<f64>::new(
        (px[0] - cx) / fx,
        (px[1] - cy) / fy,
        1.0
    )
}

// 相机坐标系到像素
#[inline]
fn cam2px(p_cam:&Vector3<f64>) -> Vector2<f64> {
    //println!("p_cam: {} ", p_cam);
    Vector2::<f64>::new(
        p_cam[0] * fx / p_cam[2] + cx,
        p_cam[1] * fy / p_cam[2] + cy
    )
}

// 检测一个点是否在图像边框内
#[inline]
fn inside(pt:&Vector2<f64>) -> bool {
  let boarder1=boarder as f64;
  return pt[0] >= boarder1 && pt[1] >= boarder1
         && pt[0] + boarder1 < width as f64 && pt[1] + boarder1 <= height as f64;
}

/*
void showEpipolarMatch(const Mat &ref, const Mat &curr, const Vector2d &px_ref, const Vector2d &px_curr) {
    Mat ref_show, curr_show;
    cv::cvtColor(ref, ref_show, CV_GRAY2BGR);
    cv::cvtColor(curr, curr_show, CV_GRAY2BGR);

    cv::circle(ref_show, cv::Point2f(px_ref(0, 0), px_ref(1, 0)), 5, cv::Scalar(0, 0, 250), 2);
    cv::circle(curr_show, cv::Point2f(px_curr(0, 0), px_curr(1, 0)), 5, cv::Scalar(0, 0, 250), 2);

    imshow("ref", ref_show);
    imshow("curr", curr_show);
    waitKey(1);
}

void showEpipolarLine(const Mat &ref, const Mat &curr, const Vector2d &px_ref, const Vector2d &px_min_curr,
                      const Vector2d &px_max_curr) {

    Mat ref_show, curr_show;
    cv::cvtColor(ref, ref_show, CV_GRAY2BGR);
    cv::cvtColor(curr, curr_show, CV_GRAY2BGR);

    cv::circle(ref_show, cv::Point2f(px_ref(0, 0), px_ref(1, 0)), 5, cv::Scalar(0, 255, 0), 2);
    cv::circle(curr_show, cv::Point2f(px_min_curr(0, 0), px_min_curr(1, 0)), 5, cv::Scalar(0, 255, 0), 2);
    cv::circle(curr_show, cv::Point2f(px_max_curr(0, 0), px_max_curr(1, 0)), 5, cv::Scalar(0, 255, 0), 2);
    cv::line(curr_show, Point2f(px_min_curr(0, 0), px_min_curr(1, 0)), Point2f(px_max_curr(0, 0), px_max_curr(1, 0)),
             Scalar(0, 255, 0), 1);

    imshow("ref", ref_show);
    imshow("curr", curr_show);
    waitKey(1);
}
*/
#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;
use std::str::{self, FromStr};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, prelude::*, core::Vector, core::KeyPoint, Result};
use kiss3d::nalgebra as na;
use na::{Matrix4, Vector2, Vector3, Isometry3, Translation3, UnitQuaternion, Quaternion};

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

  let window = "ORB features";
  highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

  // 从数据集读取数据
  let mut color_image_files:Vec<&str>=vec![];
  let mut poses_TWC:Vec<Isometry3<f64>>=vec![];
  let ref_depth=Mat::default();
  let ret = readDatasetFiles("E:\\app\\julia\\wfs2map\\src\\opencvvideo\\tests\\test_data", &mut color_image_files, &mut poses_TWC, &ref_depth);

  // 第一张图
  let ref1 = imgcodecs::imread(color_image_files[0], imgcodecs::IMREAD_GRAYSCALE)?;
  let pose_ref_TWC = poses_TWC[0];
  let init_depth = 3.0;    // 深度初始值
  let init_cov2 = 3.0;     // 方差初始值
  let depth=Mat::new_rows_cols_with_default(height, width, core::CV_64F, core::Scalar([init_depth;4])).unwrap();             // 深度图
  let depth_cov2=Mat::new_rows_cols_with_default(height, width, core::CV_64F, core::Scalar([init_cov2;4])).unwrap();         // 深度图方差

  for index in 1..color_image_files.len() {
      println!("*** loop {} ***{}", index, endl);
      let curr = imgcodecs::imread(color_image_files[index], imgcodecs::IMREAD_GRAYSCALE)?;
      if curr.data().is_null() {continue;}
      let pose_curr_TWC = poses_TWC[index];
      let pose_T_C_R = pose_curr_TWC.inverse() * pose_ref_TWC;   // 坐标转换关系： T_C_W * T_W_R = T_C_R
      update(&ref1, &curr, pose_T_C_R, &depth, &depth_cov2);
      //evaludateDepth(ref_depth, depth);
      plotDepth(&ref_depth, &depth);
      highgui::imshow("image", &curr);
      highgui::waitKey(1);
  }

  println!("estimation returns, saving depth map ...{}", endl);
  imgcodecs::imwrite_def("depth.png", &depth);
  println!("done.{}", endl);

  Ok(())
}

fn readDatasetFiles(
    path:&str,
    color_image_files:&mut Vec<&str>,
    poses:&Vec<Isometry3<f64>>,
    ref_depth:&Mat) -> bool {
    let input = File::open(String::from(path) + &String::from("/first_200_frames_traj_over_table_input_sequence.txt")).unwrap();
    let fin = BufReader::new(input);

    let mut input = String::new();
    while 0!=fin.read_line(&mut input).unwrap() {
        // 数据格式：图像文件名 tx, ty, tz, qx, qy, qz, qw ，注意是 TWC 而非 TCW
        let words = input.split_whitespace();
        let image=words.next();
        let mut data=["";7]; 
        for i in 1..words.count() {
          data[i-1]=words.next();
        }

        color_image_files.push(&path.to_string().push_str("/images/").push_str(image));
        poses.push(
            Isometry3::from_parts(Translation3::new(f64::from_str(data[0]).unwrap(), f64::from_str(data[1]).unwrap(), f64::from_str(data[2]).unwrap()),
                UnitQuaternion::from_quaternion(Quaternion::new(f64::from_str(data[6]).unwrap(), f64::from_str(data[3]).unwrap(), f64::from_str(data[4]).unwrap(), f64::from_str(data[5]).unwrap())) )
        );
    }

    // load reference depth
    let input = File::open(String::from(path) + &String::from("/depthmaps/scene_000.depth")).unwrap();
    let fin = BufReader::new(input);
    let mut ref_depth = Mat::new_rows_cols(height, width, core::CV_64F).unwrap();
    for y in 0..height {
        for x in 0..width {
            //let depth = 0;
            let mut depth = vec![];
            fin.read_until(b' ', &mut depth);
            let depth=f64::from_str(str::from_utf8(&depth).unwrap()).unwrap();
            *ref_depth.ptr(y).unwrap().offset(x as isize) = depth / 100.0;
        }
    }

    return true;
}

// 对整个深度图进行更新
fn update(ref1:&Mat, curr:&Mat, const SE3d &T_C_R, depth:&Mat, depth_cov2:&Mat) -> bool {
    for x in boarder..(width - boarder) {
        for y in boarder..(height - boarder) {
            // 遍历每个像素
            if depth_cov2.ptr(y).unwrap().offset(x) < min_cov || depth_cov2.ptr(y).unwrap().offset(x) > max_cov // 深度已收敛或发散
                continue;
            // 在极线上搜索 (x,y) 的匹配
            let mut pt_curr:Vector2<f64>;
            let mut epipolar_direction:Vector2<f64>;
            let ret = epipolarSearch(
                ref1,
                curr,
                T_C_R,
                Vector2::<f64>::new(x, y),
                depth.ptr(y).unwrap().offset(x),
                depth_cov2.ptr(y).unwrap().offset(x).sqrt(),
                pt_curr,
                epipolar_direction
            );

            if ret==false // 匹配失败
                continue;

            // 取消该注释以显示匹配
            // showEpipolarMatch(ref, curr, Vector2d(x, y), pt_curr);

            // 匹配成功，更新深度图
            updateDepthFilter(Vector2::<f64>::new(x, y), pt_curr, T_C_R, epipolar_direction, depth, depth_cov2);
        }
    }
    true
}

// 极线搜索
// 方法见书 12.2 12.3 两节
fn epipolarSearch(
    ref1:&Mat, curr:&Mat,
    T_C_R:&Isometry3<f64>, pt_ref:&Vector2<f64>,
    depth_mu:&f64, depth_cov:&f64,
    pt_curr:&mut Vector2<f64>, epipolar_direction:&mut Vector2<f64>) -> bool {
    let mut f_ref = px2cam(pt_ref);
    f_ref.normalize();
    let P_ref = f_ref.map(|e|{e*depth_mu});    // 参考帧的 P 向量

    let px_mean_curr = cam2px(T_C_R * P_ref); // 按深度均值投影的像素
    let d_min = depth_mu - 3.0 * depth_cov;
    let d_max = depth_mu + 3.0 * depth_cov;
    if d_min<0.1 {d_min = 0.1;}
    let px_min_curr = cam2px(T_C_R * (f_ref * d_min));    // 按最小深度投影的像素
    let px_max_curr = cam2px(T_C_R * (f_ref * d_max));    // 按最大深度投影的像素

    let epipolar_line = px_max_curr - px_min_curr;    // 极线（线段形式）
    epipolar_direction = &mut epipolar_line;        // 极线方向
    epipolar_direction.normalize();
    double half_length = 0.5 * epipolar_line.norm();    // 极线线段的半长度
    if half_length>100 {half_length = 100;}   // 我们不希望搜索太多东西

    // 取消此句注释以显示极线（线段）
    // showEpipolarLine( ref, curr, pt_ref, px_min_curr, px_max_curr );

    // 在极线上搜索，以深度均值点为中心，左右各取半长度
    let best_ncc = -1.0;
    let best_px_curr;
    let l=-half_length;
    while l<=half_length { // l+=sqrt(2)
        let px_curr = px_mean_curr + l * epipolar_direction;  // 待匹配点
        if !inside(px_curr) {
            continue;
        }
        // 计算待匹配点与参考帧的 NCC
        let ncc = NCC(ref, curr, pt_ref, px_curr);
        if ncc>best_ncc {
            best_ncc = ncc;
            best_px_curr = px_curr;
        }
        l += 0.7;
    }
    if best_ncc<0.85_f64 {      // 只相信 NCC 很高的匹配
        return false;
    }
    pt_curr = best_px_curr;
    return true;
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
            let value_ref:f64 = unsafe {*ref1.ptr(y + pt_ref[1] as i32).unwrap().offset((x as isize + pt_ref[0] as isize)) as f64} / 255.0;
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

/*bool updateDepthFilter(
    const Vector2d &pt_ref,
    const Vector2d &pt_curr,
    const SE3d &T_C_R,
    const Vector2d &epipolar_direction,
    Mat &depth,
    Mat &depth_cov2) {
    // 不知道这段还有没有人看
    // 用三角化计算深度
    SE3d T_R_C = T_C_R.inverse();
    Vector3d f_ref = px2cam(pt_ref);
    f_ref.normalize();
    Vector3d f_curr = px2cam(pt_curr);
    f_curr.normalize();

    // 方程
    // d_ref * f_ref = d_cur * ( R_RC * f_cur ) + t_RC
    // f2 = R_RC * f_cur
    // 转化成下面这个矩阵方程组
    // => [ f_ref^T f_ref, -f_ref^T f2 ] [d_ref]   [f_ref^T t]
    //    [ f_2^T f_ref, -f2^T f2      ] [d_cur] = [f2^T t   ]
    Vector3d t = T_R_C.translation();
    Vector3d f2 = T_R_C.so3() * f_curr;
    Vector2d b = Vector2d(t.dot(f_ref), t.dot(f2));
    Matrix2d A;
    A(0, 0) = f_ref.dot(f_ref);
    A(0, 1) = -f_ref.dot(f2);
    A(1, 0) = -A(0, 1);
    A(1, 1) = -f2.dot(f2);
    Vector2d ans = A.inverse() * b;
    Vector3d xm = ans[0] * f_ref;           // ref 侧的结果
    Vector3d xn = t + ans[1] * f2;          // cur 结果
    Vector3d p_esti = (xm + xn) / 2.0;      // P的位置，取两者的平均
    double depth_estimation = p_esti.norm();   // 深度值

    // 计算不确定性（以一个像素为误差）
    Vector3d p = f_ref * depth_estimation;
    Vector3d a = p - t;
    double t_norm = t.norm();
    double a_norm = a.norm();
    double alpha = acos(f_ref.dot(t) / t_norm);
    double beta = acos(-a.dot(t) / (a_norm * t_norm));
    Vector3d f_curr_prime = px2cam(pt_curr + epipolar_direction);
    f_curr_prime.normalize();
    double beta_prime = acos(f_curr_prime.dot(-t) / t_norm);
    double gamma = M_PI - alpha - beta_prime;
    double p_prime = t_norm * sin(beta_prime) / sin(gamma);
    double d_cov = p_prime - depth_estimation;
    double d_cov2 = d_cov * d_cov;

    // 高斯融合
    double mu = depth.ptr<double>(int(pt_ref(1, 0)))[int(pt_ref(0, 0))];
    double sigma2 = depth_cov2.ptr<double>(int(pt_ref(1, 0)))[int(pt_ref(0, 0))];

    double mu_fuse = (d_cov2 * mu + sigma2 * depth_estimation) / (sigma2 + d_cov2);
    double sigma_fuse2 = (sigma2 * d_cov2) / (sigma2 + d_cov2);

    depth.ptr<double>(int(pt_ref(1, 0)))[int(pt_ref(0, 0))] = mu_fuse;
    depth_cov2.ptr<double>(int(pt_ref(1, 0)))[int(pt_ref(0, 0))] = sigma_fuse2;

    return true;
}*/

// 后面这些太简单我就不注释了（其实是因为懒）
fn plotDepth(depth_truth:&Mat, depth_estimate:&Mat) {
    highgui::imshow("depth_truth", &(depth_truth * 0.4).into_result().unwrap().to_mat().unwrap());
    highgui::imshow("depth_estimate", &(depth_estimate * 0.4).into_result().unwrap().to_mat().unwrap());
    highgui::imshow("depth_error", &(depth_truth - depth_estimate).into_result().unwrap().to_mat().unwrap());
    highgui::waitKey(1);
}

/*void evaludateDepth(const Mat &depth_truth, const Mat &depth_estimate) {
    double ave_depth_error = 0;     // 平均误差
    double ave_depth_error_sq = 0;      // 平方误差
    int cnt_depth_data = 0;
    for (int y = boarder; y < depth_truth.rows - boarder; y++)
        for (int x = boarder; x < depth_truth.cols - boarder; x++) {
            double error = depth_truth.ptr<double>(y)[x] - depth_estimate.ptr<double>(y)[x];
            ave_depth_error += error;
            ave_depth_error_sq += error * error;
            cnt_depth_data++;
        }
    ave_depth_error /= cnt_depth_data;
    ave_depth_error_sq /= cnt_depth_data;

    cout << "Average squared error = " << ave_depth_error_sq << ", average error: " << ave_depth_error << endl;
}*/

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
fn px2cam(px:Vector2<f64) -> Vector3<f64> {
    return Vector3::<f64>::new(
        (px[0] - cx) / fx,
        (px[1] - cy) / fy,
        1.0
    );
}

// 相机坐标系到像素
#[inline]
fn cam2px(p_cam:Vector3<f64>) -> Vector2<f64> {
    return Vector2::<f64>::new(
        p_cam[0] * fx / p_cam[2] + cx,
        p_cam[1] * fy / p_cam[2] + cy
    );
}

// 检测一个点是否在图像边框内
#[inline]
fn inside(pt:&Vector2<f64>) -> bool {
  let boarder1=boarder as f64;
  return pt[0] >= boarder1 && pt[1] >= boarder1
         && pt[0] + boarder1 < width as f64 && pt[1] + boarder1 <= height as f64;
}

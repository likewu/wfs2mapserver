#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::path::Path;
use std::error::Error;

use opencv::{highgui, core, calib3d, imgcodecs, imgproc, features2d, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch, Point2d, Point2f, Point2i, Point3d}
};

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();

  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/1.png");
  let img_2_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2.png");

  let img_1 = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
  let img_2 = imgcodecs::imread(img_2_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;

  let mut keypoints_1 = Vector::<KeyPoint>::new();
  let mut keypoints_2 = Vector::<KeyPoint>::new();
  let mut good_matches = Vector::<DMatch>::new();
  find_feature_matches(&img_1, &img_2, &mut keypoints_1, &mut keypoints_2, &mut good_matches);
  println!("一共找到了{}组匹配点", good_matches.len());

  //-- 估计两张图像间运动
  let mut R = Mat::default();
  let mut t = Mat::default();
  pose_estimation_2d2d(&keypoints_1, &keypoints_2, &mut good_matches, &mut R, &mut t);

  //-- 三角化
  let mut points = Vector::<Point3d>::new();
  triangulation(&keypoints_1, &keypoints_2, &good_matches, &mut R, &mut t, &mut points);

  //-- 验证三角化点与特征点的重投影关系
  let K = Mat::new_rows_cols_with_data(3, 3, &[520.9, 0., 325.1, 0., 521.0, 249.7, 0., 0., 1.]).unwrap().clone_pointee();
  let mut img1_plot = img_1.clone();
  let mut img2_plot = img_2.clone();
  for i in 0..good_matches.len() {
    // 第一个图
    let depth1 = points.get(i).unwrap().z;
    println!("depth: {}", depth1);
    let ptcenter = keypoints_1.get(good_matches.get(i).unwrap().query_idx as usize).unwrap().pt();
    let pt1_cam:Point2d = pixel2cam(ptcenter, &K);
    imgproc::circle(&mut img1_plot, Point2i {x:ptcenter.x as i32,y:ptcenter.y as i32}, 2, get_color(depth1), 2, imgproc::LINE_8, 0);

    // 第二个图
    let K11=Mat::new_rows_cols_with_data(3, 1, &[points.get(i).unwrap().x, points.get(i).unwrap().y, points.get(i).unwrap().z]).unwrap().clone_pointee();
    let pt2_trans = &R * K11 + &t;
    let depth2:f64 = *pt2_trans.into_result()?.to_mat()?.at_2d::<f64>(2,0).unwrap();
    let ptcenter=keypoints_2.get(good_matches.get(i).unwrap().train_idx as usize).unwrap().pt();
    imgproc::circle(&mut img2_plot, Point2i {x:ptcenter.x as i32,y:ptcenter.y as i32}, 2, get_color(depth2), 2, imgproc::LINE_8, 0);
  }

  highgui::imshow("img 1", &img1_plot);
  highgui::imshow("img 2", &img2_plot);
  let _ = highgui::wait_key(0)?;

  Ok(())
}

/// 作图用
#[inline]
fn get_color(depth:f64) -> Scalar {
  let up_th:f64 = 50.;
  let low_th:f64 = 10.;
  let th_range = up_th - low_th;
  let mut depth11=depth;
  if depth11>up_th { depth11 = up_th; }
  if depth11<low_th { depth11 = low_th; }
  Scalar::new(255. * depth11 / th_range, 0., (255. * (1. - depth11 / th_range)).into(), 0.)
}

fn find_feature_matches(img_1:&Mat, img_2:&Mat,
                          keypoints_1:&mut Vector::<KeyPoint>,
                          keypoints_2:&mut Vector::<KeyPoint>,
                          good_matches:&mut Vector::<DMatch>) -> Result<(), Box<dyn Error>> {
  //-- 初始化
  let mut descriptors_1 = Mat::default();
  let mut descriptors_2 = Mat::default();
  let mut detector = features2d::ORB::create_def()?;
  let mut descriptor = features2d::ORB::create_def()?;
  // use this if you are in OpenCV2
  // Ptr<FeatureDetector> detector = FeatureDetector::create ( "ORB" );
  // Ptr<DescriptorExtractor> descriptor = DescriptorExtractor::create ( "ORB" );
  let matcher = features2d::DescriptorMatcher::create("BruteForce-Hamming")?;
  //-- 第一步:检测 Oriented FAST 角点位置
  let _=detector.detect_def(&img_1, keypoints_1);
  let _=detector.detect_def(&img_2, keypoints_2);

  //-- 第二步:根据角点位置计算 BRIEF 描述子
  let _=descriptor.compute(&img_1, keypoints_1, &mut descriptors_1);
  let _=descriptor.compute(&img_2, keypoints_2, &mut descriptors_2);

  //-- 第三步:对两幅图像中的BRIEF描述子进行匹配，使用 Hamming 距离
  let mut matches = Vector::<DMatch>::new();
  // BFMatcher matcher ( NORM_HAMMING );
  let _=matcher.train_match_def(&descriptors_1, &descriptors_2, &mut matches);
  
  //-- 第四步:匹配点对筛选
  let mut min_dist = 10000.0f32;
  let mut max_dist = 0.0f32;

  //找出所有匹配之间的最小距离和最大距离, 即是最相似的和最不相似的两组点之间的距离
  for i in 0..descriptors_1.rows() {
    let dist = matches.get(i as usize).unwrap().distance;
    if dist<min_dist {min_dist = dist;}
    if dist>max_dist {max_dist = dist;}
  }

  println!("-- Max dist : {:?} ", max_dist);
  println!("-- Min dist : {:?} ", min_dist);

  //当描述子之间的距离大于两倍的最小距离时,即认为匹配有误.但有时候最小距离会非常小,设置一个经验值30作为下限.
  for i in 0..descriptors_1.rows() {
    if matches.get(i as usize).unwrap().distance <= f32::max(2.0f32 * min_dist, 30.0f32) {
      good_matches.push(matches.get(i as usize).unwrap());
    }
  }

  Ok(())
}

fn pose_estimation_2d2d(
  keypoints_1:&Vector::<KeyPoint>,
  keypoints_2:&Vector::<KeyPoint>,
  good_matches:&mut Vector::<DMatch>,
  R:&mut Mat, t:&mut Mat) {
  // 相机内参,TUM Freiburg2
  let _K = Mat::new_rows_cols_with_data(3, 3, &[520.9, 0., 325.1, 0., 521.0, 249.7, 0., 0., 1.]).unwrap().clone_pointee();

  //-- 把匹配点转换为vector<Point2f>的形式
  let mut points1=Vector::<Point2f>::new();
  let mut points2=Vector::<Point2f>::new();

  for i in 0..good_matches.len() {
    points1.push(keypoints_1.get(good_matches.get(i).unwrap().query_idx as usize).unwrap().pt());
    points2.push(keypoints_2.get(good_matches.get(i).unwrap().train_idx as usize).unwrap().pt());
  }

  //-- 计算基础矩阵
  let fundamental_matrix;
  fundamental_matrix = calib3d::find_fundamental_mat_1(&points1, &points2, calib3d::FM_8POINT, 3.0, 0.99, &mut core::no_array()).unwrap();
  println!("fundamental_matrix is {:?}\n", fundamental_matrix.data_typed::<f64>().unwrap());

  //-- 计算本质矩阵
  let principal_point=Point2d::new(325.1, 249.7);        //相机主点, TUM dataset标定值
  let focal_length = 521;            //相机焦距, TUM dataset标定值
  let essential_matrix = calib3d::find_essential_mat_1(&points1, &points2, focal_length as f64, principal_point, calib3d::RANSAC, 0.999, 1.0, 1000, &mut core::no_array()).unwrap();
  println!("essential_matrix is {:?}\n", essential_matrix.data_typed::<f64>().unwrap());

  //-- 计算单应矩阵
  //-- 但是本例中场景不是平面，单应矩阵意义不大
  let homography_matrix;
  homography_matrix = calib3d::find_homography(&points1, &points2, &mut core::no_array(), calib3d::RANSAC, 3.0).unwrap();
  println!("homography_matrix is {:?}\n", homography_matrix.data_typed::<f64>().unwrap());

  //-- 从本质矩阵中恢复旋转和平移信息.
  let _=calib3d::recover_pose(&essential_matrix, &points1, &points2, R, t, focal_length as f64, principal_point, &mut core::no_array());
}

fn triangulation(
  keypoints_1:&Vector::<KeyPoint>,
  keypoints_2:&Vector::<KeyPoint>,
  good_matches:&Vector::<DMatch>,
  R:&Mat, t:&Mat,
  points:&mut Vector::<Point3d>) {
  let T1=Mat::new_rows_cols_with_data(3, 4, &[
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.]).unwrap().clone_pointee();
  let T2=Mat::new_rows_cols_with_data(3, 4, &[
    *R.at_2d::<f64>(0,0).unwrap(), *R.at_2d::<f64>(0,1).unwrap(), *R.at_2d::<f64>(0,2).unwrap(), *t.at_2d::<f64>(0,0).unwrap(),
    *R.at_2d::<f64>(1,0).unwrap(), *R.at_2d::<f64>(1,1).unwrap(), *R.at_2d::<f64>(1,2).unwrap(), *t.at_2d::<f64>(1,0).unwrap(),
    *R.at_2d::<f64>(2,0).unwrap(), *R.at_2d::<f64>(2,1).unwrap(), *R.at_2d::<f64>(2,2).unwrap(), *t.at_2d::<f64>(2,0).unwrap()
  ]).unwrap().clone_pointee();

  let K = Mat::new_rows_cols_with_data(3, 3, &[520.9, 0., 325.1, 0., 521.0, 249.7, 0., 0., 1.]).unwrap().clone_pointee();
  let mut pts_1=Vector::<Point2d>::new();
  let mut pts_2=Vector::<Point2d>::new();
  for m in good_matches {
    // 将像素坐标转换至相机坐标
    pts_1.push(pixel2cam(keypoints_1.get(m.query_idx as usize).unwrap().pt(), &K));
    pts_2.push(pixel2cam(keypoints_2.get(m.train_idx as usize).unwrap().pt(), &K));
  }

  let mut pts_4d=Mat::default();
  calib3d::triangulate_points(&T1, &T2, &pts_1, &pts_2, &mut pts_4d);

  // 转换成非齐次坐标
  for i in 0..pts_4d.cols() {
    let x = pts_4d.col(i).unwrap().clone_pointee();
    let x = (&x / (&x).at_2d::<f64>(3,0).unwrap()).into_result().unwrap().to_mat().unwrap(); // 归一化
    let p=Point3d::new(
      *x.at_2d::<f64>(0,0).unwrap(),
      *x.at_2d::<f64>(1,0).unwrap(),
      *x.at_2d::<f64>(2,0).unwrap()
    );
    points.push(p);
  }
}

fn pixel2cam(p:Point2f, K:&Mat) -> Point2d {
    Point2d::new
    (
      (p.x as f64 - K.at_2d::<f64>(0,2).unwrap()) / K.at_2d::<f64>(0,0).unwrap(),
      (p.y as f64 - K.at_2d::<f64>(1,2).unwrap()) / K.at_2d::<f64>(1,1).unwrap()
    )
}

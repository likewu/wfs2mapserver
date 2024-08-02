#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::path::Path;
use std::error::Error;

use opencv::{core, calib3d, imgcodecs, features2d, prelude::*, Result,
  core::{Vector, KeyPoint, DMatch, Point2d, Point2f}
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
  let _=find_feature_matches(&img_1, &img_2, &mut keypoints_1, &mut keypoints_2, &mut good_matches);
  println!("一共找到了{}组匹配点", good_matches.len());

  //-- 估计两张图像间运动
  let mut R = Mat::default();
  let mut t = Mat::default();
  pose_estimation_2d2d(&keypoints_1, &keypoints_2, &mut good_matches, &mut R, &mut t);

  //-- 验证E=t^R*scale
  let t_x = Mat::new_rows_cols_with_data(3, 3, &[
    0.0, -t.at_2d::<f64>(2,0).unwrap(), *t.at_2d::<f64>(1,0).unwrap(),
    *t.at_2d::<f64>(2,0).unwrap(), 0.0, -t.at_2d::<f64>(0,0).unwrap(),
    -t.at_2d::<f64>(1,0).unwrap(), *t.at_2d::<f64>(0,0).unwrap(), 0.0]).unwrap().clone_pointee();

  println!("t^R={:?}\n", (&t_x * &R).into_result()?.to_mat()?.data_typed::<f64>().unwrap());

  //-- 验证对极约束
  let K = Mat::new_rows_cols_with_data(3, 3, &[520.9, 0., 325.1, 0., 521.0, 249.7, 0., 0., 1.]).unwrap().clone_pointee();
  for m in good_matches {
    let pt1:Point2d = pixel2cam(keypoints_1.get(m.query_idx as usize).unwrap().pt(), &K);
    let y1 = Mat::new_rows_cols_with_data(3, 1, &[
      pt1.x, pt1.y, 1.]).unwrap().clone_pointee();
    let pt2:Point2d = pixel2cam(keypoints_2.get(m.train_idx as usize).unwrap().pt(), &K);
    let y2 = Mat::new_rows_cols_with_data(3, 1, &[
      pt2.x, pt2.y, 1.]).unwrap().clone_pointee();
    let d = (y2.t().unwrap() * &t_x * &R * y1).into_result().unwrap();
    println!("epipolar constraint = {:?}", d.to_mat()?.data_typed::<f64>().unwrap());
  }

  Ok(())
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

fn pixel2cam(p:Point2f, K:&Mat) -> Point2d {
    Point2d::new
    (
      (p.x as f64 - K.at_2d::<f64>(0,2).unwrap()) / K.at_2d::<f64>(0,0).unwrap(),
      (p.y as f64 - K.at_2d::<f64>(1,2).unwrap()) / K.at_2d::<f64>(1,1).unwrap()
    )
}

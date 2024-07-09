#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;

use opencv::{highgui, core, imgcodecs, objdetect, features2d, prelude::*, core::Vector, core::KeyPoint, Result
  
};

fn main() -> Result<()> {
  let window = "ORB features";
  highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/1.png");
  let img_2_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2.png");
  let img_1 = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
  let img_2 = imgcodecs::imread(img_2_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;

  //-- 初始化
  let mut keypoints_1 = Vector::<KeyPoint>::new();
  let mut keypoints_2 = Vector::<KeyPoint>::new();
  let mut descriptors_1 = Mat::default();
  let mut descriptors_2 = Mat::default();
  let mut detector = features2d::ORB::create_def()?;
  let mut descriptor = features2d::ORB::create_def()?;
  let mut matcher = features2d::DescriptorMatcher::create("BruteForce-Hamming")?;

  //-- 第一步:检测 Oriented FAST 角点位置
  detector.detect_def(&img_1, &mut keypoints_1);
  detector.detect_def(&img_2, &mut keypoints_2);

  //-- 第二步:根据角点位置计算 BRIEF 描述子
  descriptor.compute(&img_1, &mut keypoints_1, &mut descriptors_1);
  descriptor.compute(&img_2, &mut keypoints_2, &mut descriptors_2);

  let mut outimg1 = Mat::default();
  features2d::draw_keypoints(&img_1, &mut keypoints_1, &mut outimg1, core::Scalar::all(-1 as f64), features2d::DrawMatchesFlags::DEFAULT);
  if outimg1.size()?.width > 0 {
    highgui::imshow("ORB features", &outimg1)?;
  }

  //-- 第三步:对两幅图像中的BRIEF描述子进行匹配，使用 Hamming 距离
  let mut matches = Vector::<core::DMatch>::new();
  matcher.train_match_def(&descriptors_1, &descriptors_2, &mut matches);

  //-- 第四步:匹配点对筛选
  // 计算最小距离和最大距离
  let (max, max_index, min, min_index, sum) = matches
                     .iter()
                     .enumerate()
                     .fold((matches.get(0).unwrap(), 0, matches.get(0).unwrap(), 0, 0.0), |acc, (index,x)| {
                        let max;
                        let min;
                        let max_index;
                        let min_index;
                        let sum;
                        
                        if x.distance > acc.0.distance {
                            max = x;
                            max_index = index;
                        } else {
                            max = acc.0;
                            max_index = acc.1;
                        }
                        
                        
                        if x.distance < acc.2.distance {
                            min = x;
                            min_index = index;
                        } else {
                            min = acc.2;
                            min_index = acc.3;
                        }
                        
                        sum = acc.4 + x.distance;
                        
                        (max, max_index, min, min_index, sum)
                     });
  let min_dist = min.distance;
  let max_dist = max.distance;

  println!("-- Max dist : {}", max_dist);
  println!("-- Min dist : {}", min_dist);

  //当描述子之间的距离大于两倍的最小距离时,即认为匹配有误.但有时候最小距离会非常小,设置一个经验值30作为下限.
  let mut good_matches = Vector::<core::DMatch>::new();
  for i in 0..descriptors_1.rows() {
    if matches.get(i as usize).unwrap().distance <= f32::max(2.0 * min_dist, 30.0) {
      good_matches.push(matches.get(i as usize).unwrap());
    }
  }

  //-- 第五步:绘制匹配结果
  let mut img_match = Mat::default();
  let mut img_goodmatch = Mat::default();
  features2d::draw_matches_def(&img_1, &mut keypoints_1, &img_2, &mut keypoints_2, &matches, &mut img_match);
  features2d::draw_matches_def(&img_1, &mut keypoints_1, &img_2, &mut keypoints_2, &good_matches, &mut img_goodmatch);
  highgui::imshow("all matches", &img_match);
  highgui::imshow("good matches", &img_goodmatch);
  
  loop {
    let key = highgui::wait_key(10)?;
    if key > 0 && key != 255 {
      break;
    }
  }

  Ok(())
}

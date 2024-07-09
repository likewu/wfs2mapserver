#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;

use opencv::{core, imgcodecs, objdetect, features2d, prelude::*, core::Vector, core::KeyPoint, Result};

#[test]
fn test_orb_cv() {
  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/1.png");
  let img_2_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2.png");
  let img_1 = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR).unwrap();
  let img_2 = imgcodecs::imread(img_2_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR).unwrap();

  let mut keypoints_1 = Vector::<KeyPoint>::new();
  let mut keypoints_2 = Vector::<KeyPoint>::new();
  let mut descriptors_1 = Mat::default();
  let mut descriptors_2 = Mat::default();
  let mut detector = features2d::ORB::create_def().unwrap();
  let mut descriptor = features2d::ORB::create_def().unwrap();

  println!("{:?}", "hhhhhhhhhhhhh");

  detector.detect_def(&img_1, &mut keypoints_1);
  detector.detect_def(&img_2, &mut keypoints_2);

  descriptor.compute(&img_1, &mut keypoints_1, &mut descriptors_1);
  descriptor.compute(&img_2, &mut keypoints_2, &mut descriptors_2);

  //assert_eq!(serialized_entries[..], result[..serialized_entries.len()]);
}

#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::process::exit;
use std::path::Path;
use std::slice::from_raw_parts;

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, calib3d, imgproc, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch, Point2f, Point2i, Point3f}
};

use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  // Input Parameters:
  //
  let board_w = 12i32;
  let board_h = 12i32;
  let board_n = board_w * board_h;
  let board_sz = core::Size{width:board_w, height:board_h};
  let config_path = Path::new("E:/app/julia/Learning-OpenCV-3_examples/").join("birdseye/intrinsics.xml");
  let mut fs=core::FileStorage::new_def(config_path.to_str().unwrap(), core::FileStorage_Mode::READ as i32)?;
  let mut intrinsic=Mat::default();
  let mut distortion=Mat::default();

  intrinsic=fs.get("camera_matrix").unwrap().mat().unwrap();
  distortion=fs.get("distortion_coefficients").unwrap().mat().unwrap();
  
  if !fs.is_opened().unwrap() || intrinsic.empty() || distortion.empty() {
    println!("Error: Couldn't load intrinsic parameters from ");
    return Ok(());
  }
  fs.release();

  let img_1_path = Path::new("E:/app/julia/Learning-OpenCV-3_examples/").join("birdseye/IMG_0215L.jpg");
  let image0 = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
  let mut gray_image=Mat::default();
  let mut image=Mat::default();

  // UNDISTORT OUR IMAGE
  //
  calib3d::undistort(&image0, &mut image, &intrinsic, &distortion, &intrinsic);
  imgproc::cvt_color_def(&image, &mut gray_image, imgproc::COLOR_BGRA2GRAY);

  let mut corners=Vector::<Point2f>::new();
  let found = calib3d::find_chessboard_corners(&image, board_sz, &mut corners,
    calib3d::CALIB_CB_ADAPTIVE_THRESH | calib3d::CALIB_CB_FILTER_QUADS).unwrap();
  if !found {
    println!("Couldn't acquire checkerboard on {:?}, only found {} of {} corners\n", img_1_path.file_name().unwrap(), corners.len(), board_n);
    return Ok(());
  }

  // Get Subpixel accuracy on those corners
  //
  imgproc::corner_sub_pix(
      &gray_image,       // Input image
      &mut corners,          // Initial guesses, also output
      core::Size::new(11, 11), // Search window size
      core::Size::new(-1, -1), // Zero zone (in this case, don't use)
      core::TermCriteria::new(core::TermCriteria_COUNT|core::TermCriteria_EPS, 30, 0.1).unwrap());

  // GET THE IMAGE AND OBJECT POINTS:
  // Object points are at (r,c):
  // (0,0), (board_w-1,0), (0,board_h-1), (board_w-1,board_h-1)
  // That means corners are at: corners[r*board_w + c]
  //
  let mut objPts=[Point2f::new(0.,0.);4];
  let mut imgPts=[Point2f::new(0.,0.);4];
  objPts[0].x = 0f32;
  objPts[0].y = 0f32;
  objPts[1].x = (board_w - 1) as f32;
  objPts[1].y = 0f32;
  objPts[2].x = 0f32;
  objPts[2].y = (board_h - 1) as f32;
  objPts[3].x = (board_w - 1) as f32;
  objPts[3].y = (board_h - 1) as f32;
  imgPts[0] = corners.get(0)?;
  imgPts[1] = corners.get((board_w - 1).try_into().unwrap())?;
  imgPts[2] = corners.get(((board_h - 1) * board_w).try_into().unwrap())?;
  imgPts[3] = corners.get(((board_h - 1) * board_w + board_w - 1).try_into().unwrap())?;

  // DRAW THE POINTS in order: B,G,R,YELLOW
  //
  imgproc::circle(&mut image, imgPts[0].to::<i32>().unwrap(), 9, Scalar::from((255, 0, 0)), 3, imgproc::LINE_8, 0);
  imgproc::circle(&mut image, imgPts[1].to::<i32>().unwrap(), 9, Scalar::from((255, 0, 0)), 3, imgproc::LINE_8, 0);
  imgproc::circle(&mut image, imgPts[2].to::<i32>().unwrap(), 9, Scalar::from((255, 0, 0)), 3, imgproc::LINE_8, 0);
  imgproc::circle(&mut image, imgPts[3].to::<i32>().unwrap(), 9, Scalar::from((255, 0, 0)), 3, imgproc::LINE_8, 0);                                              

  // DRAW THE FOUND CHECKERBOARD
  //
  calib3d::draw_chessboard_corners(&mut image, board_sz, &corners, found);
  highgui::imshow("Checkers", &image);

  // FIND THE HOMOGRAPHY
  //
  let mut H = imgproc::get_perspective_transform_def(&Vector::<Point2f>::from_slice(&objPts), &Vector::<Point2f>::from_slice(&imgPts)).unwrap();
  
  // LET THE USER ADJUST THE Z HEIGHT OF THE VIEW
  //
  println!("\nPress 'd' for lower birdseye view, and 'u' for higher (it adjusts the apparent 'Z' height), Esc to exit");
  let mut Z = 15.;
  let mut birds_image=Mat::default();
  loop {
    // escape key stops
    *H.at_2d_mut::<f64>(2i32,2i32).unwrap() = Z;
    // USE HOMOGRAPHY TO REMAP THE VIEW
    //
    imgproc::warp_perspective(&image,      // Source image
                        &mut birds_image,  // Output image
                        &H,              // Transformation matrix
                        image.size()?,   // Size for output image
                        imgproc::WARP_INVERSE_MAP | imgproc::INTER_LINEAR,
                        core::BORDER_CONSTANT,
                        Scalar::all(0.) // Fill border with black
                        );
    highgui::imshow("Birds_Eye", &birds_image);
    let key = highgui::wait_key(0).unwrap() as u8 & 255;
    if key == b'u' {
      Z += 0.5;
    }
    if key == b'd' {
      Z -= 0.5;
    }
    if key == 27 {
      break;
    }
  }

  // SHOW ROTATION AND TRANSLATION VECTORS
  //
  let mut image_points=Vector::<Point2f>::new();
  let mut object_points=Vector::<Point3f>::new();
  for i in 0..4 {
      image_points.push(imgPts[i]);
      object_points.push(Point3f::new(objPts[i].x, objPts[i].y, 0.0f32));
  }
  let mut rvec=Mat::default();
  let mut tvec=Mat::default();
  let mut rmat=Mat::default();
  calib3d::solve_pnp_def(&object_points,   // 3-d points in object coordinate
               &image_points,    // 2-d points in image coordinates
               &intrinsic,       // Our camera matrix
               &Mat::default(),       // Since we corrected distortion in the
                // beginning,now we have zero distortion
                // coefficients
               &mut rvec,      // Output rotation *vector*.
               &mut tvec       // Output translation vector.
               );
  calib3d::rodrigues_def(&rvec, &mut rmat);

  // PRINT AND EXIT
  println!("rotation matrix: {:?}", rmat);
  println!("translation vector: {:?}", tvec);
  println!("homography matrix: {:?}", H);
  println!("inverted homography matrix: {:?}", H.inv_def().unwrap());

  Ok(())
}

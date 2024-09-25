#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::slice::from_raw_parts;

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, calib3d, imgproc, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch, Point2f, Point2i, Point3f}
};

use std::time::{SystemTime, UNIX_EPOCH};

fn StereoCalib(imageList:&str, nx:i32, ny:i32,
                        useUncalibrated:bool) -> Result<()> {
  let displayCorners = true;
  let showUndistorted = true;
  let mut isVerticalStereo = false; // horiz or vert cams
  const maxScale:i32 = 1i32;
  const squareSize:f32 = 1.f32;

  // actual square size
  let f = File::open(imageList).unwrap();
  let mut fin = BufReader::new(input);
  let mut i=0;
  let j=0;
  let lr=0;
  let N = nx * ny;
  let board_sz = core::Size::new(nx, ny);
  let mut imageNames=[Vector::<&str>::new();2];
  let mut boardModel=Vector::<Point3f>::new();
  let mut objectPoints=Vector::<Vector<Point3f>>::new();
  let mut points=[Vector::<Vector<Point2f>>::new();2];
  let mut corners=[Vector::<Point2f>::new();2];
  let mut found =[false, false];
  let mut imageSize;//=core::Size::new(0, 0);

  // READ IN THE LIST OF CIRCLE GRIDS:
  //
  for i in 0..ny {
    for j in 0..nx {
      boardModel.push(
          core::Point3f::new((i * squareSize) as f32, (j * squareSize) as f32, 0.f32));
    }
  }
  i = 0;
  loop {
    lr = i % 2;
    if lr == 0 {
      found[0] = false;
      found[1] = false;
    }
    let mut buf = String::new();
    if 0==fin.read_line(&mut buf).unwrap() {
      break;
    }
    let len = buf.len();
    if buf[0] == '#' {
      continue;
    }
    let img_1_path = Path::new("E:/app/julia/Learning-OpenCV-3_examples/").join(buf);
    let img = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
    if img.size()?.width == 0 { break; }
    imageSize = img.size().unwrap();
    imageNames[lr].push(buf);
    i+=1;

    // If we did not find board on the left image,
    // it does not make sense to find it on the right.
    //
    if lr == 1 && !found[0]
      continue;

    // Find circle grids and centers therein:
    for s in 1..=maxScale {
      let mut timg = img.clone();
      if s>1 {
        imgproc::resize(&img, &mut timg, core::Size{width:0, height:0}, s, s, imgproc::INTER_LINEAR);
      }
      // Just as example, this would be the call if you had circle calibration
      // boards ...
      //      found[lr] = cv::findCirclesGrid(timg, cv::Size(nx, ny),
      //      corners[lr],
      //                                      cv::CALIB_CB_ASYMMETRIC_GRID |
      //                                          cv::CALIB_CB_CLUSTERING);
      //...but we have chessboards in our images
      found[lr] = calib3d::find_chessboard_corners_def(&timg, board_sz, &mut corners[lr]).unwrap();

      if found[lr] || s==maxScale {
        let mcorners=Mat::from_exact_iter(corners[lr].iter());
        mcorners *= (1. / s);
      }
      if found[lr] {break;}
    }
    if displayCorners {
      println!("{}", buf);
      let cimg=Mat::default();
      imgproc::cvt_color(&img, &mut cimg, imgproc::COLOR_GRAY2BGR, 0);

      // draw chessboard corners works for circle grids too
      calib3d::draw_chessboard_corners(&mut cimg, core::Size::new(nx, ny), &corners[lr], found[lr]);
      highgui::imshow("Corners", &cimg);
      if key & 255 == 27 {  //Esc
        exit(-1);
      }
    } else {
      println!(".");
    }
    if lr == 1 && found[0] && found[1] {
      objectPoints.push(boardModel);
      points[0].push(corners[0]);
      points[1].push(corners[1]);
    }
  }

  // CALIBRATE THE STEREO CAMERAS
  let M1 = Mat::eye(3, 3, core::CV_64F).unwrap().to_mat().unwrap();
  let M2 = Mat::eye(3, 3, core::CV_64F).unwrap().to_mat().unwrap();
  let mut D1=Mat::default();
  let mut D2=Mat::default();
  let mut R=Mat::default();
  let mut T=Mat::default();
  let mut E=Mat::default();
  let mut F=Mat::default();
  println!("\nRunning stereo calibration ...\n");
  calib3d::stereo_calibrate(
      &objectPoints, &points[0], &points[1], &mut M1, &mut D1, &mut M2, &mut D2, imageSize, &mut R, &mut T, &mut E, &mut F,
      calib3d::CALIB_FIX_ASPECT_RATIO | calib3d::CALIB_ZERO_TANGENT_DIST |
          calib3d::CALIB_SAME_FOCAL_LENGTH,
      core::TermCriteria::new(core::TermCriteria_COUNT+core::TermCriteria_EPS, 100, 1.0e-5).unwrap());
  println!("Done! Press any key to step through images, ESC to exit\n\n");

  // CALIBRATION QUALITY CHECK
  // because the output fundamental matrix implicitly
  // includes all the output information,
  // we can check the quality of calibration using the
  // epipolar geometry constraint: m2^t*F*m1=0
  let lines=[Vector::<Point3f>::new();2];
  let avgErr = 0;
  let nframes = objectPoints.size();
  for i in 0..nframes {
    let pt0 = &points[0].get(i);
    let pt1 = &points[1].get(i);
    calib3d::undistort_points(&pt0, &mut pt0, &M1, &D1, Mat::default(), &M1);
    calib3d::undistort_points(&pt1, &mut pt1, &M2, &D2, Mat::default(), &M2);
    calib3d::compute_correspond_epilines(&pt0, 1, &F, &mut lines[0]);
    calib3d::compute_correspond_epilines(&pt1, 2, &F, &mut lines[1]);

    for j in 0..N {
      let err = (pt0[j].x * lines[1][j].x + pt0[j].y * lines[1][j].y +
                        lines[1][j].z).fabs() +
                   (pt1[j].x * lines[0][j].x + pt1[j].y * lines[0][j].y +
                        lines[0][j].z).fabs();
      avgErr += err;
    }
  }
  println!("avg err = {}", avgErr / (nframes * N));

  // COMPUTE AND DISPLAY RECTIFICATION
  //
  if showUndistorted {
    let mut R1=Mat::default();
    let mut R2=Mat::default();
    let mut P1=Mat::default();
    let mut P2=Mat::default();
    let mut map11=Mat::default();
    let mut map12=Mat::default();
    let mut map21=Mat::default();
    let mut map22=Mat::default();

    // IF BY CALIBRATED (BOUGUET'S METHOD)
    //
    if !useUncalibrated {
      calib3d::stereo_rectify_def(&M1, &D1, &M2, &D2, imageSize, &R, &T, &mut R1, &mut R2, &mut P1, &mut P2,
          &mut core::no_array());
      isVerticalStereo = (P2.at_2d<f64>(1, 3)).fabs() > (P2.at_2d<f64>(0, 3)).fabs();
      // Precompute maps for cvRemap()
      calib3d::init_undistort_rectify_map(&M1, &D1, &R1, &P1, imageSize, core::CV_16SC2, &mut map11,
          &mut map12);
      calib3d::init_undistort_rectify_map(&M2, &D2, &R2, &P2, imageSize, core::CV_16SC2, &mut map21,
          &mut map22);
    }

    // OR ELSE HARTLEY'S METHOD
    //
    else {

      // use intrinsic parameters of each camera, but
      // compute the rectification transformation directly
      // from the fundamental matrix
      let mut allpoints=[Vector::<Point2f>::new();2];
      for i in 0..nframes {
        copy(points[0][i].begin(), points[0][i].end(),
             back_inserter(allpoints[0]));
        copy(points[1][i].begin(), points[1][i].end(),
             back_inserter(allpoints[1]));
      }
      let F = calib3d::find_fundamental_mat_1(&allpoints[0], &allpoints[1], calib3d::FM_8POINT, 3.0, 0.99, &mut core::no_array()).unwrap();
      let mut H1=Mat::default();
      let mut H2=Mat::default();
      calib3d::stereo_rectify_uncalibrated(&allpoints[0], &allpoints[1], &F, imageSize,
                                    &mut H1, &mut H2, 3);
      R1 = (M1.inv() * H1 * M1).into_result().unwrap().to_mat()?;
      R2 = (M2.inv() * H2 * M2).into_result().unwrap().to_mat()?;

      // Precompute map for cvRemap()
      //
      calib3d::init_undistort_rectify_map(&M1, &D1, &R1, &P1, imageSize, core::CV_16SC2, &mut map11,
          &mut map12);
      calib3d::init_undistort_rectify_map(&M2, &D2, &R2, &P2, imageSize, core::CV_16SC2, &mut map21,
          &mut map22);
    }

    // RECTIFY THE IMAGES AND FIND DISPARITY MAPS
    //
    let pair=Mat::default();
    if !isVerticalStereo {
      pair.create_rows_cols(imageSize.height, imageSize.width * 2, core::CV_8UC3);
    } else {
      pair.create_rows_cols(imageSize.height * 2, imageSize.width, core::CV_8UC3);
    }

    // Setup for finding stereo corrrespondences
    //
    let stereo = calib3d::StereoSGBM::create(
      -64, 128, 11, 100, 1000, 32, 0, 15, 1000, 16, core::StereoSGBM::MODE_HH)
      .unwrap();

    for i in 0..nframes {
      let img1 = imgcodecs::imread(imageNames[0][i], imgcodecs::IMREAD_COLOR)?;
      let img2 = imgcodecs::imread(imageNames[1][i], imgcodecs::IMREAD_COLOR)?;
      let mut img1r=Mat::default();
      let mut img2r=Mat::default();
      let mut disp=Mat::default();
      let mut vdisp=Mat::default();
      if img1.size()?.width==0 || img2.size()?.width==0 {
        continue;
      }
      imgproc::remap_def(&img1, &mut img1r, &map11, &map12, imgproc::INTER_LINEAR);
      imgproc::remap_def(&img2, &mut img2r, &map21, &map22, imgproc::INTER_LINEAR);
      if !isVerticalStereo || !useUncalibrated {
        // When the stereo camera is oriented vertically,
        // Hartley method does not transpose the
        // image, so the epipolar lines in the rectified
        // images are vertical. Stereo correspondence
        // function does not support such a case.
        stereo.compute(&img1r, &img2r, &disp);
        core::normalize(&disp, &mut vdisp, 0., 256., core::NORM_MINMAX, core::CV_8U, &core::no_array());
        highgui::imshow("disparity", &vdisp);
      }
      if !isVerticalStereo {
        let part = pair.col_range(&core::Range::new(0, imageSize.width)).unwrap().clone_pointee();
        imgproc::cvt_color_def(&img1r, &mut part, imgproc::COLOR_GRAY2BGR);
        part = pair.col_range(&core::Range::new(imageSize.width, imageSize.width * 2));
        imgproc::cvt_color_def(&img2r, &mut part, imgproc::COLOR_GRAY2BGR);
        for j in (0..imageSize.height).step_by(16) {
          imgproc::line_def(&mut pair, Point::new(0, j), Point::new(imageSize.width * 2, j),
              core::Scalar::new(0., 255., 0., 0.));
        }
      } else {
        let part = pair.col_range(&core::Range::new(0, imageSize.height));
        imgproc::cvt_color_def(&img1r, &mut part, imgproc::COLOR_GRAY2BGR);
        part = pair.col_range(&core::Range::new(imageSize.height, imageSize.height * 2));
        imgproc::cvt_color_def(&img2r, &mut part, imgproc::COLOR_GRAY2BGR);
        for j in (0..imageSize.width).step_by(16) {
          imgproc::line_def(&mut pair, Point::new(j, 0), Point::new(j, imageSize.height * 2),
              core::Scalar::new(0., 255., 0., 0.));
        }
      }
      highgui::imshow("rectified", &pair);
      let key = highgui::wait_key(0)?;
      if key & 255 == 27 {  //Esc
        break;
      }
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  // Input Parameters:
  //
  let board_w = 9i32;
  let board_h = 6i32;
  let board_list = "E:/app/julia/Learning-OpenCV-3_examples/stereoData/example_19-03_list.txt";
  //if (argc == 4) {
  //  board_list = argv[1];
  //  board_w = atoi(argv[2]);
  //  board_h = atoi(argv[3]);
  //}
  StereoCalib(board_list, board_w, board_h, true);

  Ok(())
}

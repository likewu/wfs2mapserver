#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::process::exit;
use std::path::Path;

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, calib3d, imgproc, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch, Point2f, Point3f}
};

use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  let n_boards = 15;           // will be set by input list
  let image_sf = 0.5f64;      // image scaling factor
  let delay = 500.0f32;
  let board_w = 9i32;
  let board_h = 6i32;

  let board_n = board_w * board_h;
  let board_sz = core::Size{width:board_w, height:board_h};
  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/chessboard.avi");
  let mut capture = videoio::VideoCapture::from_file(img_1_path.to_str().unwrap(), videoio::CAP_ANY)?;
  //let mut capture = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
  let opened = videoio::VideoCapture::is_opened(&capture)?;
  if !opened {
    panic!("Unable to open default camera!");
  }

  // ALLOCATE STORAGE
  //
  let mut image_points=Vector::<Vector<Point2f>>::new();
  let mut object_points=Vector::<Vector<Point3f>>::new();

  // Capture corner views: loop until we've got n_boards successful
  // captures (all corners on the board are found).
  //
  let mut last_captured_timestamp = 0u64;
  let mut image_size=core::Size{width:0, height:0};
  while image_points.len() < n_boards {
    let mut image0 = Mat::default();
    let mut image = Mat::default();
    capture.read(&mut image0)?;
    image_size = image0.size().unwrap();
    imgproc::resize(&image0, &mut image, core::Size{width:0, height:0}, image_sf, image_sf, imgproc::INTER_LINEAR);

    // Find the board
    //
    let mut corners=Vector::<Point2f>::new();
    let found = calib3d::find_chessboard_corners_def(&image, board_sz, &mut corners).unwrap();

    // Draw it
    //
    calib3d::draw_chessboard_corners(&mut image, board_sz, &corners, found);

    // If we got a good board, add it to our data
    //
    let timestamp = SystemTime::now()
                      .duration_since(UNIX_EPOCH)
                      .unwrap()
                      .as_secs();
    println!("found:{} timestamp:{} last_captured_timestamp:{} image_points.len():{}", found, timestamp, last_captured_timestamp, image_points.len());
    if found && timestamp-last_captured_timestamp > 1 {
        last_captured_timestamp = timestamp;
        image = core::xor_mat_scalar(&image, core::Scalar::all(255.)).unwrap().to_mat()?;
        let mut mcorners=Mat::from_exact_iter(corners.clone().into_iter())?;

        // do not copy the data
        //mcorners = (&mcorners * (1.0 / image_sf)).into_result()?;

        // scale the corner coordinates
        image_points.push(corners);
        object_points.push(Vector::<Point3f>::with_capacity(board_n as usize));
        let opts = &mut object_points.get(object_points.len()-1)?;
        //object_points.remove(object_points.len())?;

        //opts.resize(board_n);
        //println!("{:?}", opts);
        for j in 0..board_n {
            opts.push(Point3f::new((j / board_w) as f32,
                                  (j % board_w) as f32, 0.0f32));
        }
        //println!("{:?}", opts);
        object_points.set(object_points.len()-1, opts.clone());
        println!("Collected our {} of {} needed chessboard images\n", image_points.len(), n_boards);
        //println!("{:?}", image_points);
        println!("{:?}", object_points);
    }
    highgui::imshow("Calibration", &image);

    // show in color if we did collect the image
    let key = highgui::wait_key(30)?;
    if key & 255 == 27 {
      //return Ok(());
      break;
    }
  }

  // END COLLECTION WHILE LOOP.
  highgui::destroy_window("Calibration");
  println!("\n\n*** CALIBRATING THE CAMERA...\n");

  // CALIBRATE THE CAMERA!
  //
  let mut intrinsic_matrix=Mat::default();
  let mut distortion_coeffs=Mat::default();
  let err = calib3d::calibrate_camera(
      &object_points, &image_points, image_size, &mut intrinsic_matrix,
      &mut distortion_coeffs, &mut core::no_array(), &mut core::no_array(),
      calib3d::CALIB_ZERO_TANGENT_DIST | calib3d::CALIB_FIX_PRINCIPAL_POINT,
      core::TermCriteria::new(core::TermCriteria_COUNT+core::TermCriteria_EPS, 30, f64::EPSILON*2.).unwrap());

  // SAVE THE INTRINSICS AND DISTORTIONS
  println!("*** DONE!\n\nReprojection error is {:?}\n\n", err);

  let intrinsic_matrix_loaded=intrinsic_matrix;
  let distortion_coeffs_loaded=distortion_coeffs;
  // Build the undistort map which we will use for all
  // subsequent frames.
  //
  let mut map1=Mat::default();
  let mut map2=Mat::default();
  calib3d::init_undistort_rectify_map(&intrinsic_matrix_loaded, &distortion_coeffs_loaded,
                            &Mat::default(), &intrinsic_matrix_loaded, image_size,
                            core::CV_16SC2, &mut map1, &mut map2);

  // Just run the camera to the screen, now showing the raw and
  // the undistorted image.
  //
  loop {
      let mut image=Mat::default();
      let mut image0=Mat::default();
      capture.read(&mut image0)?;

      if image0.size()?.width == 0 {
        break;
      }
      imgproc::remap(&image0, &mut image, &map1, &map2, imgproc::INTER_LINEAR,
          core::BORDER_CONSTANT, core::Scalar::all(0.));
      highgui::imshow("Undistorted", &image);
      let key = highgui::wait_key(30)?;
      if key & 255 == 27 {  //Esc
        break;
      }
  }

  Ok(())
}

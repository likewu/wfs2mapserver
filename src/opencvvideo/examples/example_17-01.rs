#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::process::exit;
use std::path::Path;
use std::slice::from_raw_parts;

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, video, calib3d, imgproc, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch, Point2f, Point2i, Point3f}
};

use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! phi2xy {
  ( $mat:expr,$img:expr ) => {
    core::Point::new(($img.cols() as f64 / 2. + $img.cols() as f64 / 3. * ($mat.at::<f64>(0)).unwrap().cos()).round() as i32,
        ($img.rows() as f64 / 2. - $img.cols() as f64 / 3. * ($mat.at::<f64>(0)).unwrap().sin()).round() as i32)
  };
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  let mut img=unsafe {Mat::new_rows_cols(500, 500, core::CV_8UC3).unwrap()};
  let mut kalman=video::KalmanFilter::new(2, 1, 0, core::CV_64F)?;

  // state is (phi, delta_phi) - angle and angular velocity
  // Initialize with random guess.
  //
  let mut x_k= unsafe {Mat::new_rows_cols(2, 1, core::CV_64F).unwrap()};
  core::randn(&mut x_k, &0.0, &0.1);

  // process noise
  //
  let mut w_k= unsafe {Mat::new_rows_cols(2, 1, core::CV_64F).unwrap()};

  // measurements, only one parameter for angle
  //
  let mut z_k = Mat::zeros(1, 1, core::CV_64F)?.to_mat()?;

  // Transition matrix 'F' describes relationship between
  // model parameters at step k and at step k+1 (this is
  // the "dynamics" in our model.
  //
  let F:[f64;4] = [1., 1., 0., 1.];
  kalman.set_transition_matrix(Mat::new_rows_cols_with_default(2, 2, core::CV_64F, Scalar::from_array(F))?.clone());

  // Initialize other Kalman filter parameters.
  //
  core::set_identity(&mut kalman.measurement_matrix(), Scalar::all(1.));
  core::set_identity(&mut kalman.process_noise_cov(), Scalar::all(1e-5));
  core::set_identity(&mut kalman.measurement_noise_cov(), Scalar::all(1e-1));
  core::set_identity(&mut kalman.error_cov_post(), Scalar::all(1.));

  // choose random initial state
  //
  core::randn(&mut kalman.state_post(), &0.0, &0.1);

  loop {
      // predict point position
      //
      let y_k = &kalman.predict_def().unwrap();

      // generate measurement (z_k)
      //
      core::randn(&mut z_k, &0.0,
          &kalman.measurement_noise_cov().at_2d::<f64>(0, 0)?.sqrt());
      z_k = (&kalman.measurement_matrix() * &x_k + &z_k).into_result().unwrap().to_mat()?;

      // plot points (e.g., convert
      //
      img.set_scalar(Scalar::all(0.));
      let img11=&img.clone();
      imgproc::circle(&mut img, phi2xy!(z_k,&img11), 4, Scalar::from((128, 255, 255)), 3, imgproc::LINE_8, 0);  // observed (in yellow)
      imgproc::circle(&mut img, phi2xy!(y_k,&img11), 4, Scalar::from((255, 255, 255)), 2, imgproc::LINE_8, 0);  // predicted (in white)
      imgproc::circle(&mut img, phi2xy!(x_k,&img11), 4, Scalar::from((0, 0, 255)), 3, imgproc::LINE_8, 0);  // actual to (in red)
                                                                                                     // planar co-ordinates and draw
      highgui::imshow("Kalman", &img);

      // adjust Kalman filter state
      //
      &kalman.correct(&z_k);

      // Apply the transition matrix 'F' (e.g., step time forward)
      // and also apply the "process" noise w_k
      //
      core::randn(&mut w_k, &0.0, &kalman.process_noise_cov().at_2d::<f64>(0, 0)?.sqrt());
      x_k = (&kalman.transition_matrix() * &x_k + &w_k).into_result().unwrap().to_mat()?;

      // exit if user hits 'Esc'
      let key = highgui::wait_key(100).unwrap() as u8 & 255;
      if key == 27 {
        break;
      }
  }

  Ok(())
}

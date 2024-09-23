#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::process::exit;
use std::path::Path;

use opencv::{highgui, core, imgcodecs, imgproc, objdetect, features2d, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

fn main() -> Result<()> {
  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/HandIndoorColor.jpg");
  let A = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
  if A.size()?.width == 0 { exit(1); }

  let patchSize = core::Size::new(100, 100);
  let topleft=core::Point::new(A.cols() / 2, A.rows() /2);
  let roi=core::Rect::new(topleft.x, topleft.y, patchSize.width, patchSize.height);
  let mut B = A.roi(roi).unwrap().try_clone().unwrap();

  let dft_M = core::get_optimal_dft_size(A.rows() + B.rows() - 1).unwrap();
  let dft_N = core::get_optimal_dft_size(A.cols() + B.cols() - 1).unwrap();

  let mut dft_A = Mat::zeros(dft_M, dft_N, core::CV_32F)?.to_mat()?;
  let mut dft_B = Mat::zeros(dft_M, dft_N, core::CV_32F)?.to_mat()?;

  let mut dft_A_part = dft_A.roi(core::Rect::new(0, 0, A.cols(), A.rows()))?.clone_pointee();
  let mut dft_B_part = dft_B.roi(core::Rect::new(0, 0, B.cols(), B.rows()))?.clone_pointee();

  let dft_A_part11=dft_A_part.clone();
  let dft_B_part11=dft_B_part.clone();
  A.convert_to(&mut dft_A_part, dft_A_part11.typ(), 1., -core::mean_def(&A).unwrap()[0]);
  B.convert_to(&mut dft_B_part, dft_B_part11.typ(), 1., -core::mean_def(&B).unwrap()[0]);

  let dft_A11=dft_A.clone();
  let dft_B11=dft_B.clone();
  core::dft(&dft_A11, &mut dft_A, 0, A.rows());
  core::dft(&dft_B11, &mut dft_B, 0, B.rows());

  // set the last parameter to false to compute convolution instead of correlation
  //
  let dft_A11=dft_A.clone();
  core::mul_spectrums(&dft_A11, &dft_B, &mut dft_A, 0, true);
  let dft_A11=dft_A.clone();
  core::idft(&dft_A11, &mut dft_A, core::DFT_SCALE, A.rows() + B.rows() - 1);

  let mut corr = dft_A.roi(core::Rect::new(0, 0, A.cols() + B.cols() - 1, A.rows() + B.rows() - 1))?.clone_pointee();
  let corr11=corr.clone();
  core::normalize(&corr11, &mut corr, 0., 1., core::NORM_MINMAX, corr11.typ(), &core::no_array());
  let corr11=corr.clone();
  core::pow(&corr11, 3.0, &mut corr);

  B = core::xor_mat_scalar(&B, core::Scalar::all(255.)).unwrap().to_mat()?;

  highgui::imshow("Image", &A);
  highgui::imshow("ROI", &B);

  highgui::imshow("Correlation", &corr11);

  highgui::wait_key(0)?;

  Ok(())
}

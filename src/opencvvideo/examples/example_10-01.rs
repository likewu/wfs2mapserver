#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::process::exit;
use std::path::Path;

use opencv::{highgui, core, imgcodecs, imgproc, objdetect, features2d, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

fn sum_rgb( src:&Mat, dst:&mut Mat ) {
  // Split image onto the color planes.
  //
  let mut planes=Vector::<Mat>::new();
  println!("src matrix dims: {:?} channels: {:?} depth: {:?}", src.dims(), src.channels(), src.depth());
  core::split( &src, &mut planes );
  let b = planes.get(0).unwrap();
  let g = planes.get(1).unwrap();
  let r = planes.get(2).unwrap();
  let mut s=Mat::default();

  // Add equally weighted rgb values.
  //
  core::add_weighted_def( &r, 1./3., &g, 1./3., 0.0, &mut s );
  let s11=&s.clone();
  core::add_weighted_def( &s11, 1., &b, 1./3., 0.0, &mut s );

  // Truncate values above 100.
  //
  imgproc::threshold( &s, dst, 100., 100., imgproc::THRESH_TRUNC );
}

fn main() -> Result<()> {
  // Load the image from the given file name.
  //
  let mut dst=Mat::default();
  let img_1_path = Path::new("E:/app/julia/Learning-OpenCV-3_examples/").join("faces.png");
  let src = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
  //if( src.empty() ) { cout << "can not load " << argv[1] << endl; return -1; }
  sum_rgb( &src, &mut dst);

  // Create a named window with the name of the file and
  // show the image in the window
  //
  highgui::imshow( "aaa", &dst );

  highgui::wait_key(0)?;

  Ok(())
}

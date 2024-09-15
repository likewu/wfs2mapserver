#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;

use opencv::{highgui, core, imgcodecs, imgproc, objdetect, features2d, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

fn main() -> Result<()> {
  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/HandIndoorColor.jpg");
  let src = imgcodecs::imread(img_1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
  let mut hsv = Mat::default();
  imgproc::cvt_color(&src, &mut hsv, imgproc::COLOR_BGR2HSV, 0);

  const h_ranges:[f32; 2] = [0., 180.]; // hue is [0, 180]
  const s_ranges:[f32; 2] = [0., 256.];
  //const ranges = Vector::<f32>::from_slice(&[h_ranges, s_ranges]);
  let ranges = Vector::<f32>::from_slice(&[0., 380., 0., 356.]);
  let histSize = Vector::<i32>::from_slice(&[30, 32]);
  let ch = Vector::<i32>::from_slice(&[0, 1]);

  let mut hist = Mat::default();

  // Compute the histogram
  println!("src matrix dims: {:?} channels: {:?} depth: {:?}", src.dims(), src.channels(), src.depth());
  println!("hsv matrix dims: {:?} channels: {:?} depth: {:?}", hsv.dims(), hsv.channels(), hsv.depth());
  imgproc::calc_hist(&hsv, &ch, &core::no_array(), &mut hist, &histSize, &ranges, true);
  println!("hist matrix dims: {:?} channels: {:?} depth: {:?}", hist.dims(), hist.channels(), hist.depth());
  core::normalize(&hist.clone(), &mut hist, 0., 255., core::NORM_MINMAX, -1, &core::no_array());
  println!("hist matrix dims: {:?} channels: {:?} depth: {:?}", hist.dims(), hist.channels(), hist.depth());

  let scale = 10;
  let mut hist_img=unsafe{Mat::new_rows_cols(histSize.get(0).unwrap()*scale, histSize.get(1).unwrap()*scale, core::CV_8UC3).unwrap()};

  // Draw our histogram.
  //
  for h in 0..histSize.get(0).unwrap() {
    for s in 0..histSize.get(1).unwrap() {
      let hval = hist.at_2d::<u8>(h, s).unwrap();
      imgproc::rectangle(
  &mut hist_img,
  core::Rect{x:h*scale, y:s*scale, width:scale, height:scale},
  core::Scalar::all((*hval).into()),
  -1,
  imgproc::LINE_8, 0
      );
    }
  }

  highgui::imshow("image", &src);
  highgui::imshow("H-S histogram", &hist_img);

  highgui::wait_key(0)?;

  Ok(())
}

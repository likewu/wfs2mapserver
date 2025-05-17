#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;

use opencv::{highgui, core, imgcodecs, imgproc, objdetect, features2d, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

fn main() -> Result<()> {
  let img_1_path = Path::new("E:/app/julia/wfs2map/src/opencvvideo/tests/HandIndoorColor.jpg");
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
  imgproc::calc_hist(&hsv, &ch, &core::no_array(), &mut hist, &histSize, &ranges, false);
  println!("hist matrix dims: {:?} channels: {:?} depth: {:?}", hist.dims(), hist.channels(), hist.depth());
  core::normalize(&hist.clone(), &mut hist, 0., 255., core::NORM_MINMAX, -1, &core::no_array());
  println!("hist matrix dims: {:?} channels: {:?} depth: {:?}", hist.dims(), hist.channels(), hist.depth());

  /*let scale = 10;
  let mut hist_img=unsafe{Mat::new_rows_cols(histSize.get(0).unwrap()*scale, histSize.get(1).unwrap()*scale, core::CV_8UC3).unwrap()};

  // Draw our histogram.
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

  highgui::wait_key(0)?;*/


  let client = ffi::new_blobstore_client();

  // Upload a blob.
  let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
  let mut buf = MultiBuf { chunks, pos: 0 };
  let blobid = client.put(&mut buf);
  print!("\n\n");
  println!("blobid = {blobid}");

  // Add a tag.
  client.tag(blobid, "rust");

  // Read back the tags.
  let metadata = client.metadata(blobid);
  println!("tags = {:?}", metadata.tags);

  let ret = client.maincv(1, vec!["example_13-01", "E:/app/julia/Learning-OpenCV-3_examples/fruits.jpg"]);
  println!("ret = {}", ret);

  Ok(())
}

use cxx::let_cxx_string;

#[cxx::bridge(namespace = "org::opencv")]
mod ffi {
    // Shared structs with fields visible to both languages.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("opencvvideo/include/opencv.h");

        type OpencvClient;

        fn new_blobstore_client() -> UniquePtr<OpencvClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;

        fn maincv(&self, argc: u64, argv: Vec<&str>) -> u64;
    }
}

pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}
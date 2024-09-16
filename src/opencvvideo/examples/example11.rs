#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

use lazy_static::lazy_static;
use std::sync::Mutex;

static mut image: Option<&mut Mat> = None;
static mut mask: Option<&mut Mat> = None;

fn backgroundDiff(
    I: &Mat,
    Imask: &mut Mat) {
  println!("{:?}", I);
  println!("{:?}", Imask);
}

unsafe fn adjustThresholds(argv: &[&str], img: &Mat) {
    //unsafe {
    //  backgroundDiff(img, mask.unwrap());
    //}
}

lazy_static! {
    static ref STRING: String = String::from("Hello, World");
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() -> Result<()> {
  unsafe {
      // 将`c`从内存中泄漏，变成`'static`生命周期
      image = Some(Box::leak(Box::new(Mat::default())));
      mask = Some(Box::leak(Box::new(Mat::default())));
  }

  //backgroundDiff((&image).unwrap(), (&mask).unwrap());

  let mut image11: Option<Mat> = None;
  let mut mask11: Option<Mat> = None;
  image11 = Some(Mat::default());
  mask11 = Some(Mat::default());
  backgroundDiff(&image11.unwrap(), &mut mask11.unwrap());

  &STRING;

  do_a_call();
  do_a_call();
  do_a_call();

  println!("called {}", ARRAY.lock().unwrap().len());

  Ok(())
}

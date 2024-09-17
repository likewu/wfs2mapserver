#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

use lazy_static::lazy_static;
use std::sync::Mutex;

#[cfg(not(target_env = "msvc"))]
use jemallocator;
#[cfg(not(target_env = "msvc"))]
use jemalloc_ctl::{AsName, Access};
use std::collections::HashMap;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const PROF_ACTIVE: &'static [u8] = b"prof.active\0";
const PROF_DUMP: &'static [u8] = b"prof.dump\0";
const PROFILE_OUTPUT: &'static [u8] = b"profile.out\0";

#[cfg(not(target_env = "msvc"))]
fn set_prof_active(active: bool) {
    let name = PROF_ACTIVE.name();
    name.write(active).expect("Should succeed to set prof");
}

#[cfg(not(target_env = "msvc"))]
fn dump_profile() {
    let name = PROF_DUMP.name();
    name.write(PROFILE_OUTPUT).expect("Should succeed to dump profile")
}

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
    //static ref ARRAY11: Mutex<&mut Vec<u8>> = Mutex::new(&mut vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn do_a_call22(image2222:&mut Mat) {
    *image2222=Mat::new_rows_cols_with_data(3, 3, &[520.9, 0., 325.1, 0., 521.0, 249.7, 0., 0., 1.]).unwrap().clone_pointee();
}

fn main() -> Result<()> {
  #[cfg(not(target_env = "msvc"))]
  set_prof_active(true);

  #[cfg(not(target_env = "msvc"))]
  let mut buffers: Vec<HashMap<i32, i32>> = Vec::new();
  #[cfg(not(target_env = "msvc"))]
  for _ in 0..100 {
      buffers.push(HashMap::with_capacity(1024));
  }

  #[cfg(not(target_env = "msvc"))]
  set_prof_active(false);
  #[cfg(not(target_env = "msvc"))]
  dump_profile();

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

  println!("ARRAY.lock().unwrap() {:?}", ARRAY.lock().unwrap());

  //ARRAY=Mutex::new(vec![5,6,7]);
  //*ARRAY11.lock().unwrap()=vec![8,9,0];
  println!("ARRAY.lock().unwrap() {:?}", ARRAY.lock().unwrap());


  let mut image22 = Mat::default();
  println!("image22 {:?}", image22);
  do_a_call22(&mut image22);
  println!("image22 {:?}", image22);
  let u8slice : &[f64] = unsafe{ std::slice::from_raw_parts::<f64>(image22.data() as *const f64, 9) };
  println!("image22 {:?}", u8slice);

  Ok(())
}

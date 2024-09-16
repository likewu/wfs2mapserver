#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::process::exit;
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

// Global storage
//
// Float, 3-channel images
//
static mut image: Option<&mut Mat> = None;
static mut IavgF: Option<&mut Mat> = None;
static mut IdiffF: Option<&mut Mat> = None;
static mut IprevF: Option<&mut Mat> = None;
static mut IhiF: Option<&mut Mat> = None;
static mut IlowF: Option<&mut Mat> = None;
static mut tmp: Option<&mut Mat> = None;
static mut tmp2: Option<&mut Mat> = None;
static mut mask: Option<&mut Mat> = None;

// Float, 1-channel images
//
static mut Igray: Option<&mut Vector<Mat>> = None;
static mut Ilow: Option<&mut Vector<Mat>> = None;
static mut Ihi: Option<&mut Vector<Mat>> = None;

// Byte, 1-channel image
//
static mut Imaskt: Option<&mut Mat> = None;

// Thresholds
//
static mut high_thresh:f32 = 20.0f32;  //scaling the thesholds in backgroundDiff()
static mut low_thresh:f32 = 28.0f32;

// Counts number of images learned for averaging later
//
static mut Icount:f32=0.0f32;

// I is just a sample image for allocation purposes
// (passed in for sizing)
//
unsafe fn AllocateImages( I:&Mat ) {
  unsafe {
    let sz = I.size().unwrap();
    let sz=&[sz.width, sz.height];
    IavgF = Some(Box::leak(Box::new(Mat::zeros_nd(sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    IdiffF = Some(Box::leak(Box::new(Mat::zeros_nd(sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    IprevF = Some(Box::leak(Box::new(Mat::zeros_nd(sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    IhiF = Some(Box::leak(Box::new(Mat::zeros_nd(sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    IlowF = Some(Box::leak(Box::new(Mat::zeros_nd(sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    Icount = 0.00001; // Protect against divide by zero
    tmp = Some(Box::leak(Box::new(Mat::zeros_nd( sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    tmp2 = Some(Box::leak(Box::new(Mat::zeros_nd( sz, core::CV_32FC3 ).unwrap().to_mat().unwrap())));
    Imaskt = Some(Box::leak(Box::new(Mat::new_nd( sz, core::CV_32FC1 ).unwrap())));
  }
}

// Learn the background statistics for one more frame
// I is a color sample of the background, 3-channel, 8u
//
fn accumulateBackground( I:&Mat ){
  static mut first:i32 = 1; // nb. Not thread safe
  /*I.convert_to_def( tmp.unwrap(), core::CV_32F ); // convert to float
  if( !first ){
    IavgF += tmp;
    core::absdiff( tmp, IprevF, tmp2 );
    IdiffF += tmp2;
    Icount += 1.0;
  }
  first = 0;
  IprevF = tmp;*/
}

fn setHighThreshold( scale:f32,
    Ilow11: &mut Vector<Mat>,
    Ihi11: &mut Vector<Mat> ) {
  /*IhiF = IavgF + (IdiffF * scale);
  core::split( IhiF11, Ihi11 );*/
}

fn setLowThreshold( scale:f32,
    Ilow11: &mut Vector<Mat>,
    Ihi11: &mut Vector<Mat> ) {
  /*IlowF = IavgF - (IdiffF * scale);
  core::split( IlowF11, Ilow11 );*/
}

fn createModelsfromStats() {
  /*IavgF *= (1.0/Icount);
  IdiffF *= (1.0/Icount);
  
  // Make sure diff is always something
  //
  IdiffF += core::Scalar::new( 1.0, 1.0, 1.0 );
  setHighThreshold(high_thresh.clone(), Ilow.unwrap(), Ihi.unwrap());
  setLowThreshold(low_thresh.clone(), Ilow.unwrap(), Ihi.unwrap());*/
}

// Create a binary: 0,255 mask where 255 (red) means foreground pixel
// I      Input image, 3-channel, 8u
// Imask  Mask image to be created, 1-channel 8u
//
fn backgroundDiff(
    I: &Mat,
    Imask: &mut Mat,
    Ilow11: &mut Vector<Mat>,
    Ihi11: &mut Vector<Mat>) {
  
  /*I.convert_to_def( tmp.unwrap(), core::CV_32F ); // To float
  core::split( tmp.unwrap(), Igray.unwrap() );
  
  // Channel 1
  //
  core::in_range( &Igray.unwrap().get(0).unwrap(), &Ilow11.get(0).unwrap(), &Ihi11.get(0).unwrap(), Imask );

  // Channel 2
  //
  core::in_range( &Igray.unwrap().get(1).unwrap(), &Ilow11.get(1).unwrap(), &Ihi11.get(1).unwrap(), Imaskt.unwrap() );
  core::min( Imask, Imaskt.unwrap(), Imask );

  // Channel 3
  //
  core::in_range( &Igray.unwrap().get(2).unwrap(), &Ilow11.get(2).unwrap(), &Ihi11.get(2).unwrap(), Imaskt.unwrap() );
  core::min( Imask, Imaskt.unwrap(), Imask );
*/
  // Finally, invert the results
  //
  *Imask=((&*Imask - Scalar::from(255)).into_result().unwrap().to_mat().unwrap() * -1.0).into_result().unwrap().to_mat().unwrap();
}

fn showForgroundInRed( argv: Vec<String>, img: &Mat, Igray11: &mut Vector<Mat>, mask11: &Mat) {
    let mut rawImage=Mat::default();
    core::split( img, Igray11 );
    core::max( mask11, &Igray11.get(2).unwrap(), &mut Igray11.get(2).unwrap());
    core::merge( Igray11, &mut rawImage );
    highgui::imshow( "aaa", &rawImage );
    highgui::imshow("Segmentation", mask11);
}

unsafe fn adjustThresholds(argv: Vec<String>, img: &Mat, Igray11: &mut Vector<Mat>, mask11: &mut Mat
  , Ilow11: &mut Vector<Mat>, Ihi11: &mut Vector<Mat>) {
  let key = highgui::wait_key(0).unwrap() as u8;
  while key != 27 && key != b'Q' && key != b'q'  // Esc or Q or q to exit
  {
    unsafe {
      if key == b'L' { low_thresh += 0.2;}
      if key == b'l' { low_thresh -= 0.2;}  
      if key == b'H' { high_thresh += 0.2;}
      if key == b'h' { high_thresh -= 0.2;}
    }
    unsafe {
      println!("H or h, L or l, esq or q to quit;  high_thresh = {}, low_thresh = {}", high_thresh, low_thresh);
      setHighThreshold(high_thresh.clone(), Ilow11, Ihi11);
      setLowThreshold(low_thresh.clone(), Ilow11, Ihi11);
      backgroundDiff(img, mask11, Ilow11, Ihi11);
    }
    showForgroundInRed(argv.clone(), img, Igray11, mask11);
  }
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  unsafe {
      // 将`c`从内存中泄漏，变成`'static`生命周期
      image = Some(Box::leak(Box::new(Mat::default())));
      println!("{:?}", image);
      mask = Some(Box::leak(Box::new(Mat::default())));

      Igray = Some(Box::leak(Box::new(Vector::<Mat>::with_capacity(3))));
      Ilow = Some(Box::leak(Box::new(Vector::<Mat>::with_capacity(3))));
      Ihi = Some(Box::leak(Box::new(Vector::<Mat>::with_capacity(3))));
  }

  let img_1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/tree.avi");
  let mut cap = videoio::VideoCapture::from_file(img_1_path.to_str().unwrap(), videoio::CAP_ANY)?;
  let number_to_train_on = 50;

  // FIRST PROCESSING LOOP (TRAINING):
  //
  let mut frame_count = 0;
  let mut key:u8;
  let first_frame = true;
  println!("Total frames to train on = {}", number_to_train_on); //db

  while true {
    println!("frame#: {}", frame_count);
    unsafe {
      cap.read(image.unwrap())?;
    }
    if image.unwrap().size()?.width == 0 { exit(1); } // Something went wrong, abort
    if frame_count == 0 { AllocateImages(image.unwrap());}
    unsafe {
      accumulateBackground( image.unwrap() );
    }
    highgui::imshow( "aaa", image.unwrap() );
    frame_count+=1;
    key = highgui::wait_key(7).unwrap() as u8;
    if key == 27 || key == b'q' || key == b'Q' || frame_count >= number_to_train_on {break;} //Allow early exit on space, esc, q
  }

  // We have accumulated our training, now create the models
  //
  println!("Creating the background model");
  createModelsfromStats();
  println!("Done!  Hit any key to continue into single step. Hit 'a' or 'A' to adjust thresholds, esq, 'q' or 'Q' to quit\n");
  
  // SECOND PROCESSING LOOP (TESTING):
  //
  highgui::named_window("Segmentation", highgui::WINDOW_AUTOSIZE ); //For the mask image
  key = highgui::wait_key(0).unwrap() as u8;
  while key != 27 || key == b'q' || key == b'Q' { // esc, 'q' or 'Q' to exit
    unsafe {
      cap.read(image.unwrap())?;
    }
    if image.unwrap().size()?.width == 0 {exit(0);}
    println!("{}", frame_count);
    frame_count+=1;
    unsafe {
      backgroundDiff( image.unwrap(), mask.unwrap(), Ilow.unwrap(), Ihi.unwrap() );
    }
    unsafe {
      highgui::imshow("Segmentation", mask.unwrap());
    }

    // A simple visualization is to write to the red channel
    //
    unsafe {
      showForgroundInRed( args, image.unwrap(), Igray.unwrap(), mask.unwrap());
    }
    if key == b'a' {
      println!("In adjust thresholds, 'H' or 'h' == high thresh up or down; 'L' or 'l' for low thresh up or down.");
      println!(" esq, 'q' or 'Q' to quit ");
      unsafe {
        adjustThresholds(args, image.unwrap(), Igray.unwrap(), mask.unwrap()
          , Ilow.unwrap(), Ihi.unwrap());
      }
      println!("Done with adjustThreshold, back to frame stepping, esq, q or Q to quit.");
    }
  }

  Ok(())
}

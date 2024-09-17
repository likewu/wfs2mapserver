#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::process::exit;
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, videoio, prelude::*, Result,
  core::{Vector, KeyPoint, Scalar, DMatch}
};

use lazy_static::lazy_static;
use std::sync::Mutex;

// I is just a sample image for allocation purposes
// (passed in for sizing)
//
fn AllocateImages( I:&Mat ) {
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

  // Global storage
  //
  // Float, 3-channel images
  //
  let mut image = Mat::default();
  let mut IavgF = Mat::default();
  let mut IdiffF = Mat::default();
  let mut IprevF = Mat::default();
  let mut IhiF = Mat::default();
  let mut IlowF = Mat::default();
  let mut tmp = Mat::default();
  let mut tmp2 = Mat::default();
  let mut mask = Mat::default();

  // Float, 1-channel images
  //
  let mut Igray = Vector::<Mat>::with_capacity(3);
  let mut Ilow = Vector::<Mat>::with_capacity(3);
  let mut Ihi = Vector::<Mat>::with_capacity(3);

  // Byte, 1-channel image
  //
  let mut Imaskt = Mat::default();

  // Thresholds
  //
  let mut high_thresh = 20.0f32;  //scaling the thesholds in backgroundDiff()
  let mut low_thresh = 28.0f32;

  // Counts number of images learned for averaging later
  //
  let mut Icount=0.0f32;

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
      cap.read(&mut image)?;
    }
    if image.size()?.width == 0 { exit(1); } // Something went wrong, abort
    if frame_count == 0 { AllocateImages(&image); }
    unsafe {
      accumulateBackground( &image );
    }
    highgui::imshow( "aaa", &image );
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
      cap.read(&image)?;
    }
    if image.size()?.width == 0 {exit(0);}
    println!("{}", frame_count);
    frame_count+=1;
    backgroundDiff( &image, &mask, &Ilow, &Ihi );
    highgui::imshow("Segmentation", &mask;
    
    // A simple visualization is to write to the red channel
    //
    showForgroundInRed( args, &image, &Igray, &mask );
    if key == b'a' {
      println!("In adjust thresholds, 'H' or 'h' == high thresh up or down; 'L' or 'l' for low thresh up or down.");
      println!(" esq, 'q' or 'Q' to quit ");
      adjustThresholds(args, image, Igray, mask
          , Ilow, Ihi);
      println!("Done with adjustThreshold, back to frame stepping, esq, q or Q to quit.");
    }
  }

  Ok(())
}

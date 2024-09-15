#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

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
static mut IavgF:Mat=Mat::default();
static mut IdiffF:Mat=Mat::default();
static mut IprevF:Mat=Mat::default();
static mut IhiF:Mat=Mat::default();
static mut IlowF:Mat=Mat::default();
static mut tmp:Mat=Mat::default();
static mut tmp2:Mat=Mat::default();
static mut mask:Mat=Mat::default();

// Float, 1-channel images
//
static mut Igray:Vector<Mat>=Vector::<Mat>::with_capacity(3);
static mut Ilow:Vector<Mat>=Vector::<Mat>::with_capacity(3);
static mut Ihi:Vector<Mat>=Vector::<Mat>::with_capacity(3);

// Byte, 1-channel image
//
static Imaskt:Mat=Mat::default();

// Thresholds
//
static mut high_thresh = 20.0f32;  //scaling the thesholds in backgroundDiff()
static mut low_thresh = 28.0f32;

// Counts number of images learned for averaging later
//
static mut Icount=0.0f32;

// I is just a sample image for allocation purposes
// (passed in for sizing)
//
fn AllocateImages( I:Mat ) {
  let sz = I.size();
  IavgF = Mat::zeros(sz, core::CV_32FC3 );
  IdiffF = Mat::zeros(sz, core::CV_32FC3 );
  IprevF = Mat::zeros(sz, core::CV_32FC3 );
  IhiF = Mat::zeros(sz, core::CV_32FC3 );
  IlowF = Mat::zeros(sz, core::CV_32FC3 );
  Icount = 0.00001; // Protect against divide by zero
  tmp = Mat::zeros( sz, core::CV_32FC3 );
  tmp2 = Mat::zeros( sz, core::CV_32FC3 );
  Imaskt = Mat::new( sz, core::CV_32FC1 );
}

// Learn the background statistics for one more frame
// I is a color sample of the background, 3-channel, 8u
//
fn accumulateBackground( I:Mat ){
  static first = 1; // nb. Not thread safe
  I.convertTo( tmp, CV_32F ); // convert to float
  if( !first ){
    IavgF += tmp;
    core::absdiff( tmp, IprevF, tmp2 );
    IdiffF += tmp2;
    Icount += 1.0;
  }
  first = 0;
  IprevF = tmp;
}

fn setHighThreshold( scale:&f32 ) {
  IhiF = IavgF + (IdiffF * scale);
  core::split( IhiF, Ihi );
}

fn setLowThreshold( scale:&f32 ) {
  IlowF = IavgF - (IdiffF * scale);
  core::split( IlowF, Ilow );
}

fn createModelsfromStats() {
  IavgF *= (1.0/Icount);
  IdiffF *= (1.0/Icount);
  
  // Make sure diff is always something
  //
  IdiffF += core::Scalar( 1.0, 1.0, 1.0 );
  setHighThreshold( high_thresh);
  setLowThreshold( low_thresh);
}

// Create a binary: 0,255 mask where 255 (red) means foreground pixel
// I      Input image, 3-channel, 8u
// Imask  Mask image to be created, 1-channel 8u
//
fn backgroundDiff(
    I: Mat,
    Imask: mut Mat) {
  
  I.convertTo( tmp, CV_32F ); // To float
  core::split( tmp, Igray );
  
  // Channel 1
  //
  core::inRange( Igray[0], Ilow[0], Ihi[0], Imask );

  // Channel 2
  //
  core::inRange( Igray[1], Ilow[1], Ihi[1], Imaskt );
  Imask = core::min( Imask, Imaskt );

  // Channel 3
  //
  core::inRange( Igray[2], Ilow[2], Ihi[2], Imaskt );
  Imask = core::min( Imask, Imaskt );

  // Finally, invert the results
  //
  Imask = 255 - Imask;
}

fn showForgroundInRed( argv: &[&str], img: Mat) {
    let rawImage=Mat::default();
    core::split( img, Igray );
    Igray[2] = core::max( mask, Igray[2] );
    core::merge( Igray, rawImage );
    highgui::imshow( "aaa", &rawImage );
    highgui::imshow("Segmentation", &mask);
}

fn adjustThresholds(argv: &[&str], img: Mat) {
  let mut key:u8 = 1;
  key = highgui::wait_key(0)? as u8;
  while key != 27 && key != b'Q' && key != b'q'  // Esc or Q or q to exit
  {
    if key == b'L' { low_thresh += 0.2;}
    if key == b'l' { low_thresh -= 0.2;}  
    if key == b'H' { high_thresh += 0.2;}
    if key == b'h' { high_thresh -= 0.2;}
    println!("H or h, L or l, esq or q to quit;  high_thresh = {}, low_thresh = {}", high_thresh, low_thresh);
    setHighThreshold(high_thresh);
    setLowThreshold(low_thresh);
    backgroundDiff(img, mask);
    showForgroundInRed(argv, img);
  }
}

fn main() -> Result<()> {
  unsafe {
      // 将`c`从内存中泄漏，变成`'static`生命周期
      image = Some(Box::leak(Box::new(Mat::default())));
      println!("{:?}", image);
  }

  let mut cap = videoio::VideoCapture::from_file(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/tree.avi"), videoio::CAP_ANY)?;
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
    highgui::imshow( "aaa", addr_of!(image) );
    frame_count+=1;
    key = highgui::wait_key(7)? as u8;
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
  key = highgui::wait_key(0)? as u8;
  while key != 27 || key == b'q' || key == b'Q' { // esc, 'q' or 'Q' to exit
    unsafe {
      cap.read(addr_of_mut!(image))?;
    }
    if image.size()?.width == 0 {exit(0);}
    println!(frame_count);
    frame_count+=1;
    unsafe {
      backgroundDiff( image, mask );
    }
    unsafe {
      highgui::imshow("Segmentation", &mask);
    }

    // A simple visualization is to write to the red channel
    //
    unsafe {
      showForgroundInRed( argv, image);
    }
    if key == b'a' {
      println!("In adjust thresholds, 'H' or 'h' == high thresh up or down; 'L' or 'l' for low thresh up or down.");
      println!(" esq, 'q' or 'Q' to quit ");
      unsafe {
        adjustThresholds(argv, image);
      }
      println!("Done with adjustThreshold, back to frame stepping, esq, q or Q to quit.");
    }
  }

  Ok(())
}

#![allow(clippy::integer_arithmetic)]

use std::fs::File;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::{ptr, mem, str, str::FromStr};
use std::cmp::Ord;

use nalgebra::{Vector3};
use rand::Rng;

pub mod rotation;
//pub mod random;

/// 从文件读入BAL dataset
struct BALProblem {
    pub num_cameras_:i32,
    pub num_points_:i32,
    pub num_observations_:i32,
    pub num_parameters_:i32,
    pub use_quaternions_:bool,

    pub point_index_:&mut [i32],      // 每个observation对应的point index
    pub camera_index_:&mut [i32],     // 每个observation对应的camera index
    pub observations_:&mut [f64],
    pub parameters_:&mut [f64],
}

impl BALProblem {
  fn new(filename:&str, use_quaternions:bool) -> Self {
    let input = File::open(filename).unwrap();
    let mut fptr = BufReader::new(input);


    /*if fptr.isNull() {
        println!("Error: unable to open file {}", filename);
        return;
    };*/

    // This wil die horribly on invalid files. Them's the breaks.
    let mut num_cameras_ = vec![];
    fptr.read_until(b' ', &mut num_cameras_);
    let num_cameras_=i32::from_str(str::from_utf8(&num_cameras_).unwrap().trim()).unwrap();
    let mut num_points_ = vec![];
    fptr.read_until(b' ', &mut num_points_);
    let num_points_=i32::from_str(str::from_utf8(&num_points_).unwrap().trim()).unwrap();
    let mut num_observations_ = vec![];
    fptr.read_until(b' ', &mut num_observations_);
    let num_observations_=i32::from_str(str::from_utf8(&num_observations_).unwrap().trim()).unwrap();
    
    println!("Header: {} {} {}", num_cameras_, num_points_, num_observations_);

    let mut point_index_ = [0; num_observations_];
    let mut camera_index_ = [0; num_observations_];
    let mut observations_ = [0.0; 2 * num_observations_];

    let mut num_parameters_ = 9 * num_cameras_ + 3 * num_points_;
    let mut parameters_ = [0.0;num_parameters_];

    for i:usize in 0..num_observations_ {
        let mut tmpdata = vec![];
        fptr.read_until(b' ', &mut tmpdata);
        camera_index_[i]=i32::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap();
    
        let mut tmpdata = vec![];
        fptr.read_until(b' ', &mut tmpdata);
        point_index_[i]=i32::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap();
        
        for j:usize in 0..2 {
          let mut tmpdata = vec![];
          fptr.read_until(b' ', &mut tmpdata);
          observations_[2 * i + j]=f64::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap()
        }
    }

    for i:usize in 0..num_parameters_ {
      let mut tmpdata = vec![];
      fptr.read_until(b' ', &mut tmpdata);
      parameters_[i]=f64::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap()
    }

    let use_quaternions_ = use_quaternions;
    if use_quaternions {
        // Switch the angle-axis rotations to quaternions.
        num_parameters_ = 10 * num_cameras_ + 3 * num_points_;
        let mut quaternion_parameters = [0.0;num_parameters_];
        let mut original_cursor = 0;
        let mut quaternion_cursor = 0;
        for i:usize in 0..num_cameras_ {
            rotation::AngleAxisToQuaternion(&parameters_[original_cursor..original_cursor+3], &quaternion_parameters[quaternion_cursor..quaternion_cursor+4]);
            quaternion_cursor += 4;
            original_cursor += 3;
            for j in 4..10 {
                quaternion_parameters[quaternion_cursor] = parameters_[original_cursor];
                quaternion_cursor+=1;
                original_cursor+=1;
            }
        }
        // Copy the rest of the points.
        for i:usize in 0..3 * num_points_ {
            quaternion_parameters[quaternion_cursor] = parameters_[original_cursor];
            quaternion_cursor+=1;
            original_cursor+=1;
        }
    }

    Self {
        num_cameras_,
        num_points_,
        num_observations_,
        num_parameters_,
        use_quaternions_,
        &mut point_index_,
        &mut camera_index_,
        &mut observations_,
        &mut parameters_:if use_quaternions {quaternion_parameters} else {parameters_},
    }
  }

  fn WriteToFile(&self, filename:&str) {
    let input = File::open(filename).unwrap();
    let mut fptr = BufWriter::new(input);

    /*if (fptr == NULL) {
        std::cerr << "Error: unable to open file " << filename;
        return;
    }*/

    fptr.write_fmt(format_args!("{} {} {} {}\n", self.num_cameras_, self.num_cameras_, self.num_points_, self.num_observations_)).unwrap();

    for i:usize in 0..self.num_observations_ {
        fptr.write_fmt(format_args!("{} {}", self.camera_index_[i], self.point_index_[i]));
        for j:usize in 0..2 {
            fptr.write_fmt(format_args!(" {}", self.observations_[2 * i + j]));
        }
        fptr.write_fmt(format_args!("\n"));
    }

    for i:usize in 0..num_cameras() {
        let mut angleaxis=[0.0;9];
        if self.use_quaternions_ {
            //OutPut in angle-axis format.
            rotation::QuaternionToAngleAxis(&parameters_[10*i,10*i+4], &angleaxis);
            unsafe {ptr::copy(&parameters_[10*i+4..10*i+10] as *const f64, angleaxis[3..9].as_mut_ptr(), 6 * mem::size_of::<f64>());}
        } else {
          unsafe {ptr::copy(&parameters_[9*i..9*i+9] as *const f64, angleaxis.as_mut_ptr(), 9 * mem::size_of::<f64>());}
        }
        for j:usize in 0..9 {
            fptr.write_fmt(format_args!("{}\n", angleaxis[j]));
        }
    }

    let points = self.parameters_[self.camera_block_size() * self.num_cameras_];
    for i:usize in 0..self.num_points() {
        let mut point = points[i * self.point_block_size()];
        for j in 0..self.point_block_size() {
            fptr.write_fmt(format_args!("{}\n", point[j]));
        }
    }
  }

  // Write the problem to a PLY file for inspection in Meshlab or CloudCompare
  fn WriteToPLYFile(&self, filename:&str) {
    let input = File::open(filename).unwrap();
    let mut of = BufWriter::new(input);

    of.write_fmt(format_args!("ply"));
    of.write_fmt(format_args!("\nformat ascii 1.0"));
    of.write_fmt(format_args!("\nelement vertex {}", self.num_cameras_ + self.num_points_));
    of.write_fmt(format_args!("\nproperty float x"));
    of.write_fmt(format_args!("\nproperty float y"));
    of.write_fmt(format_args!("\nproperty float z"));
    of.write_fmt(format_args!("\nproperty uchar red"));
    of.write_fmt(format_args!("\nproperty uchar green"));
    of.write_fmt(format_args!("\nproperty uchar blue"));
    of.write_fmt(format_args!("\nend_header"));

    // Export extrinsic data (i.e. camera centers) as green points.
    let mut angle_axis=[0.0;3];
    let mut center=[0.0;3];
    for i:usize in 0..self.num_cameras() {
        let camera = &self.cameras()[self.camera_block_size() as usize * i];
        self.CameraToAngelAxisAndCenter(&camera, &mut angle_axis, &mut center);
        of.write_fmt(format_args!("{} {} {} 0 255 0\n", center[0], center[1], center[2]));
    }

    // Export the structure (i.e. 3D Points) as white points.
    let points = self.parameters_[self.camera_block_size() * self.num_cameras_];
    for i:usize in 0..self.num_points() {
        let point = points[i * self.point_block_size()];
        for j in 0..self.point_block_size() {
          of.write_fmt(format_args!("{} ", point[j]));
        }
        of.write_fmt(format_args!(" 255 255 255\n"));
    }
  }

  fn CameraToAngelAxisAndCenter(&self, camera:&[f64],
                                angle_axis:&mut [f64],
                                center:&mut [f64]) {
    let angle_axis_ref;
    if self.use_quaternions_ {
        rotation::QuaternionToAngleAxis(camera, angle_axis);
        angle_axis_ref=Vector3::from_row_slice(angle_axis);
    } else {
        angle_axis_ref=Vector3::from_row_slice(camera);
    }

    // c = -R't
    let inverse_rotation = -angle_axis_ref;
    rotation::AngleAxisRotatePoint(inverse_rotation.data.as_slice(),
                         camera[(self.camera_block_size() - 6) as usize],
                         center);
    center[0] *= -1.0;
    center[1] *= -1.0;
    center[2] *= -1.0;
  }

  fn AngleAxisAndCenterToCamera(&self, angle_axis:&[f64],
                                center:&[f64],
                                camera:&mut [f64]) {
    if self.use_quaternions_ {
        rotation::AngleAxisToQuaternion(angle_axis, camera);
    } else {
        camera[0]=angle_axis[0];
        camera[1]=angle_axis[1];
        camera[2]=angle_axis[2];
    }

    // t = -R * c
    rotation::AngleAxisRotatePoint(angle_axis, center, camera[(self.camera_block_size() - 6) as usize..]);
    camera[(self.camera_block_size() - 6) as usize] *= -1.0;
    camera[(self.camera_block_size() - 5) as usize] *= -1.0;
    camera[(self.camera_block_size() - 4) as usize] *= -1.0;
  }

  fn Normalize(&self) {
    // Compute the marginal median of the geometry
    let mut tmp=[0;self.num_points_];
    let mut median=Vector3::<f64>::new(0.0,0.0,0.0);
    let points = self.mutable_points();
    for i:usize in 0..3 {
        for j:usize in 0..self.num_points_ {
            tmp[j] = points[3 * j + i] as i32;
        }
        median[i] = Median(tmp);
    }

    for i in 0..self.num_points_ {
        let point=Vector3::<f64>::new(points[(3*i) as usize],points[(3*i+1) as usize],points[(3*i+2) as usize]);
        tmp[i] = (point - median).lp_norm(1) as i32;
    }

    let median_absolute_deviation = Median(tmp);

    // Scale so that the median absolute deviation of the resulting
    // reconstruction is 100

    let scale = 100.0 / median_absolute_deviation;

    // X = scale * (X - median)
    for i in 0..self.num_points_ {
        let mut point=Vector3::<f64>::new(points[(3*i) as usize],points[(3*i+1) as usize],points[(3*i+2) as usize]);
        point = scale * (point - median);
        points[(3*i) as usize]=point[0];
        points[(3*i+1) as usize]=point[1];
        points[(3*i+2) as usize]=point[2];
    }

    let cameras = self.mutable_cameras();
    let mut angle_axis=[0.0;3];
    let mut center=[0.0;3];
    for i in 0..self.num_cameras_ {
        let camera = &cameras[(self.camera_block_size() * i) as usize..];
        self.CameraToAngelAxisAndCenter(&camera, &mut angle_axis, &mut center);
        // center = scale * (center - median)
        center[0] = scale * (center[0] - median[0]);
        center[1] = scale * (center[1] - median[1]);
        center[2] = scale * (center[2] - median[2]);
        self.AngleAxisAndCenterToCamera(&angle_axis, &center, &mut camera);
    }
  }

  fn Perturb(&self, rotation_sigma:&f64,
             translation_sigma:&f64,
             point_sigma:&f64) {
    assert!(*point_sigma >= 0.0);
    assert!(*rotation_sigma >= 0.0);
    assert!(*translation_sigma >= 0.0);

    let points = &mut self.mutable_points();
    if *point_sigma > 0. {
        for i in 0..self.num_points_ {
            PerturbPoint3(point_sigma, &mut points[(3 * i) as usize..]);
        }
    }

    for i in 0..self.num_cameras_ {
        let mut camera = &mut self.mutable_cameras()[(self.camera_block_size() * i) as usize..];

        let mut angle_axis=[0.0;3];
        let mut center=[0.0;3];
        // Perturb in the rotation of the camera in the angle-axis
        // representation
        self.CameraToAngelAxisAndCenter(&camera, &mut angle_axis, &mut center);
        if *rotation_sigma > 0.0 {
            PerturbPoint3(rotation_sigma, &mut angle_axis);
        }
        self.AngleAxisAndCenterToCamera(&angle_axis, &center, &mut camera);

        if *translation_sigma > 0.0 {
            PerturbPoint3(translation_sigma, &mut camera[(self.camera_block_size() - 6) as usize..]);
        }
    }
  }

  pub fn camera_block_size(&self) -> i32 { if self.use_quaternions_ {10} else {9} }

  pub fn point_block_size(&self) -> i32 { 3 }

  pub fn num_cameras(&self) -> i32 { self.num_cameras_ }

  pub fn num_points(&self) -> i32 { self.num_points_ }

  pub fn num_observations(&self) -> i32 { self.num_observations_ }

  pub fn num_parameters(&self) -> i32 { self.num_parameters_ }

  pub fn point_index(&self) -> &[i32] { self.point_index_ }

  pub fn camera_index(&self) -> &[i32] { self.camera_index_ }

  pub fn observations(&self) -> &[f64] { self.observations_ }

  pub fn parameters(&self) -> &[f64] { self.parameters_ }

  pub fn cameras(&self) -> &[f64] { self.parameters_ }

  pub fn points(&self) -> &[f64] { self.parameters_ + self.camera_block_size() * num_cameras_ }

  /// camera参数的起始地址
  pub fn mutable_cameras(&self) -> &mut [f64] { &mut self.parameters_ }

  pub fn mutable_points(&self) -> &[f64] { &mut self.parameters_[self.camera_block_size() * num_cameras_] }

  pub fn mutable_camera_for_observation(&self, i:&i32) -> &[f64] {
      self.mutable_cameras()[self.camera_index_[i] * self.camera_block_size()]
  }

  pub fn mutable_point_for_observation(&self, i:&i32) -> &[f64] {
      self.mutable_points()[self.point_index_[i] * self.point_block_size()]
  }

  pub fn camera_for_observation(&self, i:&i32) -> &[f64] {
      self.cameras()[self.camera_index_[i] * self.camera_block_size()]
  }

  pub fn point_for_observation(&self, i:&i32) -> &[f64] {
      self.points()[self.point_index_[i] * self.point_block_size()]
  }
}

fn PerturbPoint3(sigma:&f64, point:&mut [f64]) {
    let mut rng = rand::thread_rng();
    for i in 0..3 {
        point[i] += rng.gen::<f64>() * sigma;
    }
}

fn Median(mut data:Vec<i32>) -> f64 {
  let n = data.len();
  let mid_point = n / 2;
  let data=data.as_mut_slice();
  pdqselect::select(data, mid_point as usize);
  //let (lesser, median, greater) = data.select_nth_unstable_by(mid_point, |a, b| b.cmp(a));
  data[mid_point] as f64
}

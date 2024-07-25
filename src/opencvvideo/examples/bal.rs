#![allow(clippy::integer_arithmetic)]

use std::fs::File;
use std::io::BufReader;

use nalgebra::{Vector2, Vector3, UnitQuaternion, Quaternion};

pub mod rotation;

/// 从文件读入BAL dataset
struct BALProblem {
    num_cameras_:i32,
    num_points_:i32,
    num_observations_:i32,
    num_parameters_:i32,
    use_quaternions_:bool,

    point_index_:&[i32],      // 每个observation对应的point index
    camera_index_:&[i32],     // 每个observation对应的camera index
    observations_:&[f64],
    parameters_:&[f64],
}

impl BALProblem {
  fn new(filename:&str, use_quaternions:bool) {
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

    for i in 0..num_observations_ {
        let mut tmpdata = vec![];
        fptr.read_until(b' ', &mut tmpdata);
        camera_index_[i]=i32::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap();
    
        let mut tmpdata = vec![];
        fptr.read_until(b' ', &mut tmpdata);
        point_index_[i]=i32::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap()
        
        for j in 0..2 {
          let mut tmpdata = vec![];
          fptr.read_until(b' ', &mut tmpdata);
          observations_[2 * i + j]=f64::from_str(str::from_utf8(&tmpdata).unwrap().trim()).unwrap()
        }
    }

    for i in 0..num_parameters_ {
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
        for i in 0..num_cameras_ {
            rotation::AngleAxisToQuaternion(parameters_[original_cursor..original_cursor+3], quaternion_parameters[quaternion_cursor..quaternion_cursor+4]);
            quaternion_cursor += 4;
            original_cursor += 3;
            for j in 4..10 {
                quaternion_parameters[quaternion_cursor] = parameters_original_cursor];
                quaternion_cursor+=1;
                original_cursor+=1;
            }
        }
        // Copy the rest of the points.
        for i in 0..3 * num_points_ {
            quaternion_parameters[quaternion_cursor] = parameters_original_cursor];
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
        point_index_,
        camera_index_,
        observations_,
        parameters_:if use_quaternions {quaternion_parameters} else {parameters_},
    }
  }

  fn WriteToFile(filename:&str) {
    let input = File::open(filename).unwrap();
    let mut fptr = BufWriter::new(input);

    /*if (fptr == NULL) {
        std::cerr << "Error: unable to open file " << filename;
        return;
    }*/

    fprintf(fptr, "%d %d %d %d\n", num_cameras_, num_cameras_, num_points_, num_observations_);

    for i in 0..num_observations_ {
        fprintf(fptr, "%d %d", camera_index_[i], point_index_[i]);
        for j in 0..2 {
            fprintf(fptr, " %g", observations_[2 * i + j]);
        }
        fprintf(fptr, "\n");
    }

    for i in 0..num_cameras() {
        double angleaxis[9];
        if use_quaternions_ {
            //OutPut in angle-axis format.
            QuaternionToAngleAxis(parameters_ + 10 * i, angleaxis);
            memcpy(angleaxis + 3, parameters_ + 10 * i + 4, 6 * sizeof(double));
        } else {
            memcpy(angleaxis, parameters_ + 9 * i, 9 * sizeof(double));
        }
        for j in 0..9 {
            fprintf(fptr, "%.16g\n", angleaxis[j]);
        }
    }

    const double *points = parameters_ + camera_block_size() * num_cameras_;
    for i in 0..num_points() {
        const double *point = points + i * point_block_size();
        for j in 0..point_block_size() {
            fprintf(fptr, "%.16g\n", point[j]);
        }
    }
  }

  // Write the problem to a PLY file for inspection in Meshlab or CloudCompare
  fn WriteToPLYFile(const std::string &filename) {
    std::ofstream of(filename.c_str());

    of << "ply"
       << '\n' << "format ascii 1.0"
       << '\n' << "element vertex " << num_cameras_ + num_points_
       << '\n' << "property float x"
       << '\n' << "property float y"
       << '\n' << "property float z"
       << '\n' << "property uchar red"
       << '\n' << "property uchar green"
       << '\n' << "property uchar blue"
       << '\n' << "end_header" << std::endl;

    // Export extrinsic data (i.e. camera centers) as green points.
    double angle_axis[3];
    double center[3];
    for i in 0..num_cameras() {
        const double *camera = cameras() + camera_block_size() * i;
        CameraToAngelAxisAndCenter(camera, angle_axis, center);
        of << center[0] << ' ' << center[1] << ' ' << center[2]
           << " 0 255 0" << '\n';
    }

    // Export the structure (i.e. 3D Points) as white points.
    const double *points = parameters_ + camera_block_size() * num_cameras_;
    for i in 0..num_points() {
        const double *point = points + i * point_block_size();
        for (int j = 0; j < point_block_size(); ++j) {
            of << point[j] << ' ';
        }
        of << " 255 255 255\n";
    }
  }

  fn CameraToAngelAxisAndCenter(const double *camera,
                                            double *angle_axis,
                                            double *center) {
    VectorRef angle_axis_ref(angle_axis, 3);
    if use_quaternions_ {
        QuaternionToAngleAxis(camera, angle_axis);
    } else {
        angle_axis_ref = ConstVectorRef(camera, 3);
    }

    // c = -R't
    Eigen::VectorXd inverse_rotation = -angle_axis_ref;
    AngleAxisRotatePoint(inverse_rotation.data(),
                         camera + camera_block_size() - 6,
                         center);
    VectorRef(center, 3) *= -1.0;
  }

  fn AngleAxisAndCenterToCamera(const double *angle_axis,
                                            const double *center,
                                            double *camera) const {
    ConstVectorRef angle_axis_ref(angle_axis, 3);
    if use_quaternions_ {
        AngleAxisToQuaternion(angle_axis, camera);
    } else {
        VectorRef(camera, 3) = angle_axis_ref;
    }

    // t = -R * c
    AngleAxisRotatePoint(angle_axis, center, camera + camera_block_size() - 6);
    VectorRef(camera + camera_block_size() - 6, 3) *= -1.0;
  }

  fn Normalize() {
    // Compute the marginal median of the geometry
    std::vector<double> tmp(num_points_);
    Eigen::Vector3d median;
    double *points = mutable_points();
    for i in 0..3 {
        for j in 0..num_points_ {
            tmp[j] = points[3 * j + i];
        }
        median(i) = Median(&tmp);
    }

    for i in 0..num_points_ {
        VectorRef point(points + 3 * i, 3);
        tmp[i] = (point - median).lpNorm<1>();
    }

    const double median_absolute_deviation = Median(&tmp);

    // Scale so that the median absolute deviation of the resulting
    // reconstruction is 100

    const double scale = 100.0 / median_absolute_deviation;

    // X = scale * (X - median)
    for i in 0..num_points_ {
        VectorRef point(points + 3 * i, 3);
        point = scale * (point - median);
    }

    double *cameras = mutable_cameras();
    double angle_axis[3];
    double center[3];
    for i in 0..num_cameras_ {
        double *camera = cameras + camera_block_size() * i;
        CameraToAngelAxisAndCenter(camera, angle_axis, center);
        // center = scale * (center - median)
        VectorRef(center, 3) = scale * (VectorRef(center, 3) - median);
        AngleAxisAndCenterToCamera(angle_axis, center, camera);
    }
  }

  fn Perturb(const double rotation_sigma,
                         const double translation_sigma,
                         const double point_sigma) {
    assert(point_sigma >= 0.0);
    assert(rotation_sigma >= 0.0);
    assert(translation_sigma >= 0.0);

    double *points = mutable_points();
    if point_sigma > 0 {
        for i in 0..num_points_ {
            PerturbPoint3(point_sigma, points + 3 * i);
        }
    }

    for i in 0..num_cameras_ {
        double *camera = mutable_cameras() + camera_block_size() * i;

        double angle_axis[3];
        double center[3];
        // Perturb in the rotation of the camera in the angle-axis
        // representation
        CameraToAngelAxisAndCenter(camera, angle_axis, center);
        if (rotation_sigma > 0.0) {
            PerturbPoint3(rotation_sigma, angle_axis);
        }
        AngleAxisAndCenterToCamera(angle_axis, center, camera);

        if translation_sigma > 0.0
            PerturbPoint3(translation_sigma, camera + camera_block_size() - 6);
    }
  }
}

fn FscanfOrDie(FILE *fptr, const char *format, T *value) {
    int num_scanned = fscanf(fptr, format, value);
    if (num_scanned != 1)
        std::cerr << "Invalid UW data file. ";
}

fn PerturbPoint3(const double sigma, double *point) {
    for (int i = 0; i < 3; ++i)
        point[i] += RandNormal() * sigma;
}

fn Median(std::vector<double> *data) -> f64 {
  int n = data->size();
  std::vector<double>::iterator mid_point = data->begin() + n / 2;
  std::nth_element(data->begin(), mid_point, data->end());
  return *mid_point;
}

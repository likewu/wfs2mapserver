#![allow(clippy::integer_arithmetic)]

using ceres_solver::{LossFunction, AutoDiffCostFunction};

/// 从文件读入BAL dataset
struct SnavelyReprojectionError {
  observed_x:f64,
  observed_y:f64,
}

impl<T> SnavelyReprojectionError {
  fn new(observation_x:f64, observation_y:f64) {
    Self {
      observed_x:observation_x,
      observed_y:observation_y,
    }
  }

  fn operator()(camera:&T, point:&T, residuals:&T) -> bool {
      // camera[0,1,2] are the angle-axis rotation
      T predictions[2];
      CamProjectionWithDistortion(camera, point, predictions);
      residuals[0] = predictions[0] - T(observed_x);
      residuals[1] = predictions[1] - T(observed_y);

      true
  }

  // camera : 9 dims array
  // [0-2] : angle-axis rotation
  // [3-5] : translateion
  // [6-8] : camera parameter, [6] focal length, [7-8] second and forth order radial distortion
  // point : 3D location.
  // predictions : 2D predictions with center of the image plane.
  static inline bool CamProjectionWithDistortion(const T *camera, const T *point, T *predictions) {
      // Rodrigues' formula
      T p[3];
      AngleAxisRotatePoint(camera, point, p);
      // camera[3,4,5] are the translation
      p[0] += camera[3];
      p[1] += camera[4];
      p[2] += camera[5];

      // Compute the center fo distortion
      T xp = -p[0] / p[2];
      T yp = -p[1] / p[2];

      // Apply second and fourth order radial distortion
      const T &l1 = camera[7];
      const T &l2 = camera[8];

      T r2 = xp * xp + yp * yp;
      T distortion = T(1.0) + r2 * (l1 + l2 * r2);

      const T &focal = camera[6];
      predictions[0] = focal * distortion * xp;
      predictions[1] = focal * distortion * yp;

      true
  }

  fn Create(observed_x:f64, observed_y:f64) -> LossFunction {
      new ceres::AutoDiffCostFunction<SnavelyReprojectionError, 2, 9, 3>(
          Self {observed_x, observed_y})
  }
}

#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::fmt::format;
use std::path::Path;
use std::str::{self, FromStr};
use std::fs::{File};
use std::io::{BufReader, BufRead};

use opencv::{highgui, core, imgcodecs, objdetect, features2d, prelude::*, core::Vector, core::KeyPoint, Result};
use nalgebra::{Matrix2, Point2, Vector2, Vector3, Isometry3, Translation3, UnitQuaternion, Quaternion};
use ceres_solver::{CostFunctionType, NllsProblem, SolverOptions, SolverOptionsBuilder, LossFunction};

pub mod bal;
pub mod errfun;

fn main() {
  let args: Vec<String> = env::args().collect();

  BALProblem bal_problem("E:\\app\\julia\\wfs2map\\src\\opencvvideo\\tests\\problem-16-22106-pre.txt");
  bal_problem.Normalize();
  bal_problem.Perturb(0.1, 0.5, 0.5);
  bal_problem.WriteToPLYFile("initial.ply");
  SolveBA(bal_problem);
  bal_problem.WriteToPLYFile("final.ply");
}

fn SolveBA(bal_problem::&bal::BALProblem) {
    let point_block_size = bal_problem.point_block_size();
    let camera_block_size = bal_problem.camera_block_size();
    let mut points = bal_problem.mutable_points();
    let mut cameras = bal_problem.mutable_cameras();

    // Observations is 2 * num_observations long array observations
    // [u_1, u_2, ... u_n], where each u_i is two dimensional, the x
    // and y position of the observation.
    let observations = bal_problem.observations();
    let mut problem = NllsProblem::new();

    for i in 0..bal_problem.num_observations() {
        // Each observation corresponds to a pair of a camera and a point
        // which are identified by camera_index()[i] and point_index()[i]
        // respectively.
        let camera = cameras[camera_block_size * bal_problem.camera_index()[i]];
        let point = points[point_block_size * bal_problem.point_index()[i]];

        let cost: CostFunctionType = Box::new(
          move |parameters: &[&[f64]],
                residuals: &mut [f64],
                mut jacobians: Option<&mut [Option<&mut [&mut [f64]]>]>| {
              assert_eq!(parameters.len(), 2);
              // camera[0,1,2] are the angle-axis rotation
              let predictions=[0.0;2];
              // Each Residual block takes a point and a camera as input
              // and outputs a 2 dimensional Residual
              CamProjectionWithDistortion(parameters[0], parameters[1], predictions);
              residuals[0] = predictions[0] - observations[2 * i + 0];
              residuals[1] = predictions[1] - observations[2 * i + 1];
              assert_eq!(residuals.len(), 2);
              true
          },
        );
        problem = problem
          .residual_block_builder()
          .set_cost(cost, 2)
          .set_lost(LossFunction::huber(1.0))
          .add_parameter(camera)
          .add_parameter(point)
          .build_into_problem()
          .unwrap()
          .0;
    }

    // show some information here ...
    println!("bal problem file loaded...");
    println!("bal problem have {} cameras and {} points. ", bal_problem.num_cameras(), bal_problem.num_points());
    println!("Forming {} observations. ", bal_problem.num_observations());

    println!("Solving ceres BA ... ");
    // Solve the problem
    let solution = problem.solve(&linear_solver_type::default()
      .linear_solver_type(ceres_solver::solver::SPARSE_SCHUR)
      .minimizer_progress_to_stdout(true).build().unwrap()).unwrap();
    println!("Brief summary: {:?}", solution.summary);
    // Getting parameter values
    let a = solution.parameters[0][0];
    println!(solution.summary.FullReport() + "\n");
}

// camera : 9 dims array
// [0-2] : angle-axis rotation
// [3-5] : translateion
// [6-8] : camera parameter, [6] focal length, [7-8] second and forth order radial distortion
// point : 3D location.
// predictions : 2D predictions with center of the image plane.
#[inline]
fn CamProjectionWithDistortion(camera:&[f64], point:&[f64], predictions:&mut [f64]) {
    // Rodrigues' formula
    let mut p=[0.0;3];
    rotation::AngleAxisRotatePoint(camera, point, &mut p);
    // camera[3,4,5] are the translation
    p[0] += camera[3];
    p[1] += camera[4];
    p[2] += camera[5];

    // Compute the center fo distortion
    let xp = -p[0] / p[2];
    let yp = -p[1] / p[2];

    // Apply second and fourth order radial distortion
    let l1 = &camera[7];
    let l2 = &camera[8];

    let r2 = xp * xp + yp * yp;
    let distortion = 1.0 + r2 * (l1 + l2 * r2);

    let focal = &camera[6];
    predictions[0] = focal * distortion * xp;
    predictions[1] = focal * distortion * yp;

    return true;
}

#![allow(clippy::integer_arithmetic)]
//#![cfg(ocvrs_has_module_objdetect)]

use std::env;
use std::time::{SystemTime};
use std::str::{self, FromStr};

use opencv::{core::RNG, prelude::RNGTrait};
use nalgebra::{Matrix3, Vector3};

use rand::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let ar = 1.0;
  let br = 2.0;
  let cr = 1.0;         // 真实参数值
  let mut ae = 2.0;
  let mut be = -1.0;
  let mut ce = 5.0;        // 估计参数值
  let N = 100;                                 // 数据点
  let w_sigma = 1.0;                        // 噪声Sigma值
  let inv_sigma = 1.0 / w_sigma;
  let mut rng=RNG::default().unwrap();                                 // OpenCV随机数产生器

  let mut x_data:Vec<f64>=vec![];
  let mut y_data:Vec<f64>=vec![];      // 数据
  for i in 0..N {
    let x = i as f64 / 100.0;
    x_data.push(x);
    y_data.push((ar * x * x + br * x + cr).exp() + rng.gaussian(w_sigma * w_sigma).unwrap());
  }

  // 开始Gauss-Newton迭代
  let iterations = 100;    // 迭代次数
  let mut cost = 0.0;
  let mut lastCost = 0.0;  // 本次迭代的cost和上一次迭代的cost

  let now = SystemTime::now();
  for iter in 0..iterations {
    let mut H:Matrix3<f64> = Matrix3::<f64>::zeros();             // Hessian = J^T W^{-1} J in Gauss-Newton
    let mut b:Vector3<f64> = Vector3::<f64>::zeros();             // bias
    cost = 0.0;

    for i in 0..N {
      let xi = x_data[i];
      let yi = y_data[i];  // 第i个数据点
      let error = yi - (ae * xi * xi + be * xi + ce).exp();
      let mut J:Vector3<f64>=Vector3::<f64>::zeros(); // 雅可比矩阵
      J[0] = -xi * xi * (ae * xi * xi + be * xi + ce).exp();  // de/da
      J[1] = -xi * (ae * xi * xi + be * xi + ce).exp();  // de/db
      J[2] = -(ae * xi * xi + be * xi + ce).exp();  // de/dc

      H += inv_sigma * inv_sigma * J * J.transpose();
      b += -inv_sigma * inv_sigma * error * J;

      cost += error * error;
    }

    // 求解线性方程 Hx=b
    //let dx:Vector3<f64> = H.ldlt().solve(b);
    let dx:Vector3<f64> = H.cholesky().unwrap().solve(&b);
    if dx[0].is_nan() {
      println!("result is nan!");
      break;
    }

    if iter>0 && cost>=lastCost {
      println!("cost: {}>= last cost: {}, break.", cost, lastCost);
      break;
    }

    ae += dx[0];
    be += dx[1];
    ce += dx[2];

    lastCost = cost;

    println!("total cost: {}, \t\tupdate: {}\t\testimated params: {},{},{}", cost, dx.transpose(), ae, be, ce);
  }

  let time_used = now.elapsed().unwrap().as_secs();
  println!("solve time cost = {} seconds. ", time_used);

  println!("estimated abc = {}, {}, {}", ae, be, ce);
}

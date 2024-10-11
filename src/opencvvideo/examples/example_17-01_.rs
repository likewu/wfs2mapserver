//! Operation of a Bayesian state estimator in a simple example.
//!
//! Use a Kalman filter (estimator) with one state and constant noises.

use nalgebra::{Matrix2, Matrix1x2, Matrix1, Vector2, Vector1};

use bayes_estimate::models::{ExtendedLinearObserver, ExtendedLinearPredictor, KalmanState};
use bayes_estimate::noise::CorrelatedNoise;

use rand::RngCore;
use rand_distr::{Distribution, StandardNormal, Uniform};

fn main() {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let uniform01: Uniform<f64> = Uniform::new(0f64, (1e-1f64).sqrt());
    let uniform02: Uniform<f64> = Uniform::new(0f64, (1e-5f64).sqrt());

    // Se tup the initial state and covariance
    let mut estimate = KalmanState {
        x: Vector2::new(-1.6030947e-10, 0.015813505), // initially at 10
        X: Matrix2::new(1., 0., 0., 1.),  // with no uncertainty
    };
    let mut x_k = estimate.x;
    println!("Initial x{:.9} X{:.5}", estimate.x, estimate.X);

    // Make a state prediction with simple linear model and additive noise
    let my_predict_model = Matrix2::new(1., 1., 0., 1.);
    let my_predict_noise = CorrelatedNoise {
        Q: Matrix2::new(1e-5, 0., 0., 1e-5),
    };
    let mut i = 0;
    loop {
      if i>10 {break;}
      //let predicted_x = &my_predict_model * estimate.x;
      estimate
          .predict(&x_k, &my_predict_model, &my_predict_noise)
          .unwrap();
      println!("Predict x{:.9} X{:.5}", estimate.x, estimate.X);

      // Make an observation that we appear to be at 11
      //let z = Vector1::new(-0.20583938);
      let aa=rng1.clone();
      let mut ur: Vec<f64> = uniform01.sample_iter(aa).take(1).collect();

      // Observation models and observation noise
      let my_observe_model = Matrix1x2::new(1., 0.);
      let my_observe_noise = CorrelatedNoise {
          Q: Matrix1::new(1e-1),
      };
      let z = &my_observe_model * &x_k + Vector1::new(ur[0]);
      println!("z {:.9} ur {:.9}", &z, &ur[0]);

      // Make the position observation using the difference of what we observe to predicted observation
      let innovation = z - &my_observe_model * &estimate.x;
      estimate
          .observe_innovation(&innovation, &my_observe_model, &my_observe_noise)
          .unwrap();
      println!("Observe x{:.9} X{:.5}", estimate.x, estimate.X);

      let bb=rng2.clone();
      let mut ur: Vec<f64> = uniform02.sample_iter(bb).take(2).collect();
      x_k = &my_predict_model * &x_k + Vector2::new(ur[0], ur[1]);
      i += 1;
    }
}

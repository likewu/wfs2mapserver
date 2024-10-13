//! Operation of a Bayesian state estimator in a simple example.
//!
//! Use a Kalman filter (estimator) with one state and constant noises.

use nalgebra::{Matrix1, Vector1};

use bayes_estimate::models::{ExtendedLinearObserver, ExtendedLinearPredictor, KalmanState};
use bayes_estimate::noise::CorrelatedNoise;

fn main() {
    // Se tup the initial state and covariance
    let mut estimate = KalmanState {
        x: Vector1::new(10.), // initially at 10
        X: Matrix1::new(0.),  // with no uncertainty
    };
    println!("Initial x{:.1} X{:.2}", estimate.x, estimate.X);

    // Make a state prediction with simple linear model and additive noise
    let my_predict_model = Matrix1::new(1.);
    let my_predict_noise = CorrelatedNoise {
        Q: Matrix1::new(1.),
    };
    let predicted_x = my_predict_model * estimate.x;
    estimate
        .predict(&predicted_x, &my_predict_model, &my_predict_noise)
        .unwrap();
    println!("Predict x{:.1} X{:.2}", estimate.x, estimate.X);

    // Make an observation that we appear to be at 11
    let z = Vector1::new(11.);

    // Observation models and observation noise
    let my_observe_model = Matrix1::new(1.);
    let my_observe_noise = CorrelatedNoise {
        Q: Matrix1::new(2.),
    };

    // Make the position observation using the difference of what we observe to predicted observation
    let innovation = z - &my_observe_model * &estimate.x;
    estimate
        .observe_innovation(&innovation, &my_observe_model, &my_observe_noise)
        .unwrap();
    println!("Observe x{:.1} X{:.2}", estimate.x, estimate.X);
}

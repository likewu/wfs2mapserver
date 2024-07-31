use rand::prelude::*;

#[inline]
fn RandDouble() -> f64 {
    rand::random::<f64>()
}

#[inline]
fn RandNormal() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    y
}

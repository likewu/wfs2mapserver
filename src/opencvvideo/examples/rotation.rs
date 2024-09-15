#![allow(clippy::integer_arithmetic)]

#[inline]
fn DotProduct(x:&[f64], y:&[f64]) -> f64 {
    x[0] * y[0] + x[1] * y[1] + x[2] * y[2]
}

#[inline]
fn CrossProduct(x:&[f64], y:&[f64], result:&mut [f64]) {
    result[0] = x[1] * y[2] - x[2] * y[1];
    result[1] = x[2] * y[0] - x[0] * y[2];
    result[2] = x[0] * y[1] - x[1] * y[0];
}

//http://www.euclideanspace.com/maths/geometry/rotations/conversions/eulerToQuaternion/index.htm
//https://github.com/ceres-solver/ceres-solver/blob/master/include/ceres/rotation.h
// Convert a value in combined axis-angle representation to a quaternion.
// The value angle_axis is a triple whose norm is an angle in radians,
// and whose direction is aligned with the axis of rotation,
// and quaternion is a 4-tuple that will contain the resulting quaternion.
// The implementation may be used with auto-differentiation up to the first
// derivative, higher derivatives may have unexpected results near the origin.
#[inline]
pub fn AngleAxisToQuaternion(angle_axis:&[f64], quaternion:&mut [f64]) {
    let a0 = &angle_axis[0];
    let a1 = &angle_axis[1];
    let a2 = &angle_axis[2];
    let theta_squared = a0 * a0 + a1 * a1 + a2 * a2;

    if theta_squared > f64::EPSILON {
        let theta = theta_squared.sqrt();
        let half_theta = theta * 0.5;
        let k = half_theta.sin() / theta;
        quaternion[0] = half_theta.cos();
        quaternion[1] = a0 * k;
        quaternion[2] = a1 * k;
        quaternion[3] = a2 * k;
    } else { // in case if theta_squared is zero
        let k=0.5;
        quaternion[0] = 1.0;
        quaternion[1] = a0 * k;
        quaternion[2] = a1 * k;
        quaternion[3] = a2 * k;
    }
}

#[inline]
pub fn QuaternionToAngleAxis(quaternion:&[f64], angle_axis:&mut [f64]) {
    let q1 = quaternion[1];
    let q2 = quaternion[2];
    let q3 = quaternion[3];
    let sin_squared_theta = q1 * q1 + q2 * q2 + q3 * q3;

    // For quaternions representing non-zero rotation, the conversion
    // is numercially stable
    if sin_squared_theta > f64::EPSILON {
        let sin_theta = sin_squared_theta.sqrt();
        let cos_theta = quaternion[0];

        // If cos_theta is negative, theta is greater than pi/2, which
        // means that angle for the angle_axis vector which is 2 * theta
        // would be greater than pi...

        let two_theta = 2.0 * if cos_theta < 0.0
                              { (-sin_theta).atan2(-cos_theta) }
                              else { sin_theta.atan2(cos_theta) };
        let k = two_theta / sin_theta;

        angle_axis[0] = q1 * k;
        angle_axis[1] = q2 * k;
        angle_axis[2] = q3 * k;
    } else {
        // For zero rotation, sqrt() will produce NaN in derivative since
        // the argument is zero. By approximating with a Taylor series,
        // and truncating at one term, the value and first derivatives will be
        // computed correctly when Jets are used..
        let k=2.0;
        angle_axis[0] = q1 * k;
        angle_axis[1] = q2 * k;
        angle_axis[2] = q3 * k;
    }
}

#[inline]
pub fn AngleAxisRotatePoint(angle_axis:&[f64], pt:&[f64], result:&mut [f64]) {
    let theta2 = DotProduct(angle_axis, angle_axis);
    if theta2 > f64::EPSILON {
        // Away from zero, use the rodriguez formula
        //
        //   result = pt costheta +
        //            (w x pt) * sintheta +
        //            w (w . pt) (1 - costheta)
        //
        // We want to be careful to only evaluate the square root if the
        // norm of the angle_axis vector is greater than zero. Otherwise
        // we get a division by zero.
        //
        let theta = theta2.sqrt();
        let costheta = theta.cos();
        let sintheta = theta.sin();
        let theta_inverse = 1.0 / theta;

        let w = [angle_axis[0] * theta_inverse,
                 angle_axis[1] * theta_inverse,
                 angle_axis[2] * theta_inverse];

        // Explicitly inlined evaluation of the cross product for
        // performance reasons.
        /*const T w_cross_pt[3] = { w[1] * pt[2] - w[2] * pt[1],
                                  w[2] * pt[0] - w[0] * pt[2],
                                  w[0] * pt[1] - w[1] * pt[0] };*/
        let mut w_cross_pt=[0.0;3];
        CrossProduct(&w, pt, &mut w_cross_pt);

        let tmp = DotProduct(&w, pt) * (1.0 - costheta);
        //    (w[0] * pt[0] + w[1] * pt[1] + w[2] * pt[2]) * (T(1.0) - costheta);

        result[0] = pt[0] * costheta + w_cross_pt[0] * sintheta + w[0] * tmp;
        result[1] = pt[1] * costheta + w_cross_pt[1] * sintheta + w[1] * tmp;
        result[2] = pt[2] * costheta + w_cross_pt[2] * sintheta + w[2] * tmp;
    } else {
        // Near zero, the first order Taylor approximation of the rotation
        // matrix R corresponding to a vector w and angle w is
        //
        //   R = I + hat(w) * sin(theta)
        //
        // But sintheta ~ theta and theta * w = angle_axis, which gives us
        //
        //  R = I + hat(w)
        //
        // and actually performing multiplication with the point pt, gives us
        // R * pt = pt + w x pt.
        //
        // Switching to the Taylor expansion near zero provides meaningful
        // derivatives when evaluated using Jets.
        //
        // Explicitly inlined evaluation of the cross product for
        // performance reasons.
        /*const T w_cross_pt[3] = { angle_axis[1] * pt[2] - angle_axis[2] * pt[1],
                                  angle_axis[2] * pt[0] - angle_axis[0] * pt[2],
                                  angle_axis[0] * pt[1] - angle_axis[1] * pt[0] };*/
        let mut w_cross_pt=[0.0;3];
        CrossProduct(angle_axis, pt, &mut w_cross_pt);

        result[0] = pt[0] + w_cross_pt[0];
        result[1] = pt[1] + w_cross_pt[1];
        result[2] = pt[2] + w_cross_pt[2];
    }
}

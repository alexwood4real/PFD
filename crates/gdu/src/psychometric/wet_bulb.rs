/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * wet_bulb.rs
 *
 * Description:
 * The wet bulb temperature is the lowest temperature air can
 * reach by evaporating water.
 *
 * Formula:
 * Stull (2011)
 *
 * wb = T * arctan[0.151977 * (RH + 8.313659)^(1/2)]  (a)
 *    + arctan(T + RH) - arctan(RH - 1.676331)        (b)
 *    + 0.00391838(RH)^(3/2) * arctan(0.023101 * RH)  (c)
 *    - 4.686035
 *
 * Source: https://open.library.ubc.ca/media/stream/pdf/52383/1.0041967/1
 **************************************************************/
use libm::{atanf, powf};

pub fn calculate_wet_bulb(h: f32, t: f32) -> f32 {
    let a: f32 = t * atanf(0.151977 * powf(h + 8.313659, 0.5));
    let b: f32 = atanf(t + h) - atanf(h - 1.676331);
    let c: f32 = 0.00391838 * powf(h, 1.5) * atanf(0.023101 * h);

    a + b + c - 4.686035
}

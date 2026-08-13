/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * altitude.rs
 *
 * Description:
 * The altitude can be measured from barometric pressure. It was
 * the whole insperation for starting this project actually, I
 * don't know why I didn't think of calculating it first.
 *
 * Formula:
 *         /     /    p    \^0.190284  \
 * alt =  | 1 - |-----------|           | * 145366.45
 *         \     \ 1013.25 /           /
 *
 * Source: https://www.weather.gov/media/epz/wxcalc/pressureAltitude.pdf
 **************************************************************/
use libm::powf;

pub fn calculate_altitude(p: f32) -> f32 {
    /* convert to hPa */
    let hpa: f32 = p / 100.0;

    (1.0 - powf(hpa / 1013.25, 0.190284)) * 145366.45
}

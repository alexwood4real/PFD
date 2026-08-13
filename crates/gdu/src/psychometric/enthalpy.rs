/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * enthalpy.rs
 *
 * Description:
 * The sum of the internal energy plus the product of pressure
 * and volume.
 *
 * Formula:
 * ASHRAE equation
 *
 * h = 1.006T + w(2501 + 1.86T)
 *
 * Source: https://www.grc.nasa.gov/www/k-12/BGP/enthalpy.html
 **************************************************************/

pub fn calculate_enthalpy(t: f32, mr: f32) -> f32 {
    let mr_conv: f32 = mr / 1000.0; // kg / kg

    (1.006 * t) + mr_conv * (2501.0 + (1.86 * t))
}

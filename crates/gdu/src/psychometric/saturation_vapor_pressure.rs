/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * saturation_vapor_pressure.rs
 *
 * Description:
 * The SVP of water is the pressure exerted by a vapor in
 * thermodynamic equilibrium with its condended phases in a
 * closed system. At this point, the rate of evaporation equals
 * the rate of condensation
 *
 * Formula:
 * Buck Equation where T is the temperature in °C
 * and pressure is in hPa
 *
 *                   / 17.502 * T \
 * es = 6.1121 * exp|--------------|
 *                   \ 240.97 + T /
 *
 * Source: https://cires1.colorado.edu/~voemel/vp.html
 **************************************************************/
use libm::expf;

pub fn calculate_saturation_vapor_pressure(temp: f32) -> f32 {
    let n: f32 = 17.502 * temp;
    let d: f32 = 240.97 + temp;

    let e: f32 = expf(n / d);

    (6.1121) * e
}

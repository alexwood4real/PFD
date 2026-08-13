/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * vapor_pressure_deficit.rs
 *
 * Description:
 * The VPD is the difference between the amount of moisture
 * that is actually in the air vs what that air could hold
 * at saturation.
 *
 * Formula:
 * vpd = svp - vp
 *
 * Source: https://blog.ucs.org/carly-phillips/what-is-vapor-pressure-deficit-vpd-and-what-is-its-connection-to-wildfires/
 **************************************************************/

pub fn calculate_vapor_pressure_deficit(svp: f32, vp: f32) -> f32 {
    svp - vp
}

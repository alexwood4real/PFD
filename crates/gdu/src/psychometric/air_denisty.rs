/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * air_density.rs
 *
 * Description:
 * Air denisty is the mass of air molecules ina given space.
 * It decreases with altitude, and is affected by temperature,
 * humidity, and pressure
 *
 * Formula:
 * Moist Air Equation
 *
 *         /  P - e  \       e
 * rho =  |-----------| + |----|
 *         \   RdT   /      RvT
 *
 * Sources:
 * https://pilotinstitute.com/air-density-guide/
 * https://blogs.millersville.edu/adecaria/files/2021/11/esci340-Lesson02-Thermodynamics.pdf
 **************************************************************/

pub fn calculate_air_density(p: f32, t: f32, vp: f32) -> f32 {
    let t_kelvin: f32 = t + 273.15;
    let e: f32 = vp * 100.0; // vapor pressure in pascals
    let rd: f32 = 287.058;
    let rv: f32 = 461.495;

    ((p - e) / (rd * t_kelvin)) + (e / (rv * t_kelvin))
}

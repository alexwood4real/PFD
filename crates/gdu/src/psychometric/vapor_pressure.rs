/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * vapor_pressure.rs
 *
 * Description:
 * The vapor pressure of a liquid is the equilibrium pressure of
 * a vapor above its liquid; that is, the pressure of the vapor resulting
 * from evaporation of a liquid above a sample of the liquid in
 * a closed container
 *
 * Formula:
 * Relative Humidity Equation where e is the vapor pressure
 * and svp is the saurated vapor pressure
 *
 *        e
 * rh = |---| => e = rh * svp
 *       svp
 *
 * Sources:
 * https://www.weather.gov/media/epz/wxcalc/vaporPressure.pdf
 * https://www.chem.purdue.edu/gchelp/liquids/vpress.html
 **************************************************************/

pub fn calculate_vapor_pressure(svp: f32, h: f32) -> f32 {
    let rh: f32 = h / 100.0;

    rh * svp
}

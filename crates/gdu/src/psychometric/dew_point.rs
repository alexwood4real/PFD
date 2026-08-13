/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * dew_point.rs
 *
 * Description:
 * The dew point is the temperature at which air becomes fully
 * saturated with water vapor, meaning it can no longer hold
 * all the moisture in the form of invisible gas.
 *
 * Formula:
 * Apply the inverse of Bucks and using the Magnus equation with
 * the coefficients from Bucks
 *
 *        243.5 * gamma
 * Td = |---------------| s.t. gamma = inverse Bucks
 *       17.502 - gamma
 *
 * Sources:
 * https://kestrelinstruments.com/blog/what-is-the-dew-point-and-how-is-it-calculated?srsltid=AfmBOooW7d5SuaML7pszppxQW43wVH_WPiYK5nuf2-IfX9Wiznjga3R0
 * https://www.weather.gov/media/epz/wxcalc/rhTdFromWetBulb.pdf
 **************************************************************/
use libm::logf;

pub fn calculate_dew_point(vp: f32) -> f32 {
    let gamma: f32 = logf(vp / 6.1121);

    (240.97 * gamma) / (17.502 - gamma)
}

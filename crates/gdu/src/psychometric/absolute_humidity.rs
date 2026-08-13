/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * absolute_humidity.rs
 *
 * Description:
 * The abolsute humidity is the actual amount of water vapor
 * in the air regardless of temperature.
 *
 * Formula:
 * Ideal Gas Law
 *
 *        vp * 2.1674
 * AH = |-------------|
 *        273.15 + T
 *
 * Sources:
 * https://www.weather.gov/lmk/humidity
 * https://carnotcycle.wordpress.com/2012/08/04/how-to-convert-relative-humidity-to-absolute-humidity/
 **************************************************************/

pub fn calculate_absolute_humidity(temp: f32, vp: f32) -> f32 {
    /* x100 to adjust hPa -> Pascals */
    100.0 * (vp * 2.1674) / (273.15 + temp)
}

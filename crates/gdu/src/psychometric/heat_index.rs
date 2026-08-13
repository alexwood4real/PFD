/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * heat_index.rs
 *
 * Description:
 * "It's not the heat, it's the humidity that gets ya"
 * The heat index is the apparent temperature. It is what the
 * temperature feels like to the human body when relative
 * humidity is combined with the air temperature.
 *
 * Formula:
 * Rothfusz Regression (NOAA)
 *
 * T = temp °F
 * RH = humidity %
 *
 * First, the simple formula is computed:
 *
 * hi = 0.5 * ( T + 61.0 + 1.2*(T - 68.0) + (RH * 0.094)
 *
 * if the average of this heat index and the T°F > 80°F,
 * then the following regression is applied with appropriate adjustments:
 *
 * hi = -42.379 + 2.04901523 * T + 10.14333127 * RH         (a)
 *     - 0.22475541 * T * RH - 0.00683783 * T^2             (b)
 *     - 0.05481717 * RH^2 + 0.00122874 * T^2 * RH          (c)
 *     + 0.00085282 * T * RH^2 - 0.00000199 * T^2 * RH^2    (d)
 *
 * if RH < 13% and T in (80-112)°F, then the adjustment shall be subtracted off
 *
 *        / 13 - RH \         / 17 - |T - 95|  \
 * ADJ = |-----------| * sqrt|-----------------|
 *        \    4    /         \       17       /
 *
 * else, if RH > 85% and T in (80-87)°F, then the adjustment shall be added
 *
 *        / RH - 85 \     / 87 - T  \
 * ADJ = |-----------| * |-----------|
 *        \    10   /     \    5    /
 *
 * Sources:
 * https://www.weather.gov/ama/heatindex
 * https://www.wpc.ncep.noaa.gov/html/heatindex_equation.shtml
 **************************************************************/
use libm::{powf, sqrtf};

pub fn calculate_heat_index(t: f32, rh: f32) -> f32 {
    let temp_f: f32 = (t * 1.8) + 32.0;

    /* calculate simple equation first */
    let mut hi_temp: f32 = 0.5 * (temp_f + 61.0 + (1.2 * (temp_f - 68.0)) + (rh * 0.094));

    let avg: f32 = (hi_temp + temp_f) / 2.0;

    let hi: f32 = if avg >= 80.0 {
        /* perform regression */
        let a: f32 = -42.379 + (2.04901523 * temp_f) + (10.14333127 * rh);
        let b: f32 = -0.22475541 * temp_f * rh - 0.00683783 * powf(temp_f, 2.0);
        let c: f32 = -0.05481717 * powf(rh, 2.0) + 0.00122874 * powf(temp_f, 2.0) * rh;
        let d: f32 =
            0.00085282 * temp_f * powf(rh, 2.0) - 0.00000199 * powf(temp_f, 2.0) * powf(rh, 2.0);

        hi_temp = a + b + c + d;

        let adj: f32 = if rh < 13.0 && temp_f > 80.0 && temp_f < 112.0 {
            /* RH < 13%, T between (80, 112)°F */
            -((13.0 - rh) / 4.0) * sqrtf((17.0 - (temp_f - 95.0).abs()) / 17.0)
        } else if rh > 85.0 && temp_f > 80.0 && temp_f < 87.0 {
            /* RH > 85%, T between (80, 87)°F */
            ((rh - 85.0) / 10.0) * ((87.0 - temp_f) / 5.0)
        } else {
            0.0
        }; /* end of adjustment if-else statement */

        /* adjusted heat index */
        hi_temp + adj
    } else {
        /* average temperature is below 80°F */
        hi_temp
    }; /* end of heat index if-else statement */

    /* resulting heat index */
    hi
}

/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * specific_humidity.rs
 *
 * Description:
 * The Specific Humidity is the ratio of the mass of the vapor
 * in a sample, to the mass of the moist air in the sample of air
 *
 * Formula:
 *
 *        /   mr   \
 * sh =  |----------|
 *        \ 1 + mr /
 *
 * Source: https://vortex.plymouth.edu/~stmiller/stmiller_content/Publications/AtmosRH_Equations_Rev.pdf
 **************************************************************/

pub fn calculate_specific_humidity(mr: f32) -> f32 {
    /* divide by 1000 to go from g/kg -> kg/kg */
    let mr_conv: f32 = mr / 1000.0;

    mr_conv / (1.0 + mr_conv)
}

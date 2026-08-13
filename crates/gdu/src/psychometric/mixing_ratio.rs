/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * mixing_ratio.rs
 *
 * Description:
 * The Mixing Ratio is defined as the ratio of the mass of water
 * vapor to the mass of dry air in a volume of air. Measured
 * in grams of water per kg of dry air
 *
 * Formula:
 *
 *               /   vp   \
 * w = 621.97 * |----------|
 *               \ P - vp /
 *
 * Sources:
 * https://www.weather.gov/media/epz/wxcalc/mixingRatio.pdf
 * https://www.sciencedirect.com/topics/earth-and-planetary-sciences/mixing-ratio
 **************************************************************/

pub fn calculate_mixing_ratio(pres: f32, vp: f32) -> f32 {
    let hpa: f32 = pres / 100.0;

    621.97 * (vp / (hpa - vp))
}

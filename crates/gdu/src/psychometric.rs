/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * psychometric.rs
 *
 * Description:
 * Main driver in order to calculate everything given from
 * BME 280 sensor. Gets temp, humidity, pressure, and others
 **************************************************************/

use serde::Serialize;

pub mod absolute_humidity;
pub mod air_denisty;
pub mod altitude;
pub mod dew_point;
pub mod enthalpy;
pub mod heat_index;
pub mod mixing_ratio;
pub mod saturation_vapor_pressure;
pub mod specific_humidity;
pub mod vapor_pressure;
pub mod vapor_pressure_deficit;
pub mod wet_bulb;

pub use absolute_humidity::calculate_absolute_humidity;
pub use air_denisty::calculate_air_density;
pub use altitude::calculate_altitude;
pub use dew_point::calculate_dew_point;
pub use enthalpy::calculate_enthalpy;
pub use heat_index::calculate_heat_index;
pub use mixing_ratio::calculate_mixing_ratio;
pub use saturation_vapor_pressure::calculate_saturation_vapor_pressure;
pub use specific_humidity::calculate_specific_humidity;
pub use vapor_pressure::calculate_vapor_pressure;
pub use vapor_pressure_deficit::calculate_vapor_pressure_deficit;
pub use wet_bulb::calculate_wet_bulb;

/* holds initial sensor reading data */
pub struct SensorData {
    pub temperature: f32, /* °C */
    pub pressure: f32,    /* Pascals */
    pub humidity: f32,    /* % water vapor in air  */
}

/* holds calculated data */
#[derive(Serialize)]
pub struct PsychometricData {
    pub temperature: f32,               /* °C */
    pub pressure: f32,                  /* Pascals */
    pub humidity: f32,                  /* % water vapor in air  */
    pub altitude: f32,                  /* Altitude (ft) */
    pub saturation_vapor_pressure: f32, /* Hectopascals (hPa) */
    pub vapor_pressure: f32,            /* Vapor Pressure (hPa) */
    pub dew_point: f32,                 /* Dew Point °C */
    pub vapor_pressure_deficit: f32,    /* Vapor Pressure Deficit (hpa) */
    pub absolute_humidity: f32,         /* Absolute Humidity (g/m^3) */
    pub mixing_ratio: f32,              /* Mixing Ratio (g water / kg dry air) */
    pub specific_humidity: f32,         /* Specific Humidity (kg / kg) */
    pub air_density: f32,               /* Air Density (kg / m^3) */
    pub enthalpy: f32,                  /* Enthalpy (kJ / kg dry air) */
    pub wet_bulb: f32,                  /* Wet Bulb (°C) */
    pub heat_index: f32,                /* Heat Index (°F) */
}

/* calculate data from initial sensor readings */
impl SensorData {
    pub fn calculate(&self) -> Option<PsychometricData> {
        /* unwrap values from sensor */
        let temp: f32 = self.temperature;
        let pres: f32 = self.pressure;
        let hum: f32 = self.humidity;

        /* collect data */
        let alt: f32 = calculate_altitude(pres);
        let svp: f32 = calculate_saturation_vapor_pressure(temp);
        let vp: f32 = calculate_vapor_pressure(svp, hum);
        let dp: f32 = calculate_dew_point(vp);
        let vpd: f32 = calculate_vapor_pressure_deficit(svp, vp);
        let ah: f32 = calculate_absolute_humidity(temp, vp);
        let mr: f32 = calculate_mixing_ratio(pres, vp);
        let sh: f32 = calculate_specific_humidity(mr);
        let ad: f32 = calculate_air_density(pres, temp, vp);
        let ent: f32 = calculate_enthalpy(temp, mr);
        let wb: f32 = calculate_wet_bulb(hum, temp);
        let hi: f32 = calculate_heat_index(temp, hum);

        Some(PsychometricData {
            temperature: temp,
            pressure: pres,
            humidity: hum,
            altitude: alt,
            saturation_vapor_pressure: svp,
            vapor_pressure: vp,
            dew_point: dp,
            vapor_pressure_deficit: vpd,
            absolute_humidity: ah,
            mixing_ratio: mr,
            specific_humidity: sh,
            air_density: ad,
            enthalpy: ent,
            wet_bulb: wb,
            heat_index: hi,
        })
    }
}

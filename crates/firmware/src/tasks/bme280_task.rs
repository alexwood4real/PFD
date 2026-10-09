use bme280::spi::BME280;
use embassy_time::{Duration, Timer};
use gdu::psychometric::SensorData;
use log::{error, info};

use crate::drivers::bme280_driver::BME280Spi;

#[embassy_executor::task]
pub async fn bme280_task(mut bme280: BME280<BME280Spi>) -> ! {
    /* BME280 task */
    loop {
        /* read data from single sample */
        let measurements = match bme280.measure(&mut embassy_time::Delay) {
            Ok(m) => m,
            Err(e) => {
                error!("Error reading measurements: {:?}", e);
                Timer::after(Duration::from_secs(1)).await;
                continue;
            }
        };

        /* convert sample to readable data */
        let sensor_data: SensorData = SensorData {
            temperature: measurements.temperature,
            pressure: measurements.pressure,
            humidity: measurements.humidity,
        };

        /* compute weather data from sensor data */
        if let Some(weather_data) = sensor_data.calculate() {
            /* DEBUG print weather data */
            info!("Temperature: {} C", weather_data.temperature);
            info!("Pressure: {} hPa", weather_data.pressure);
            info!("Humidity: {} %", weather_data.humidity);
            info!("Altitude: {} ft", weather_data.altitude);
            info!(
                "Saturation Vapor Pressure: {} hPa",
                weather_data.saturation_vapor_pressure
            );
            info!("Vapor Pressure: {} hPa", weather_data.vapor_pressure);
            info!("Dew Point: {} C", weather_data.dew_point);
            info!(
                "Vapor Pressure Deficit: {} hPa",
                weather_data.vapor_pressure_deficit
            );
            info!(
                "Absolute Humidity: {} g/m^3",
                weather_data.absolute_humidity
            );
            info!(
                "Mixing Ratio: {} g water / kg dry air",
                weather_data.mixing_ratio
            );
            info!(
                "Specific Humidity: {} g water / kg dry air",
                weather_data.specific_humidity
            );
            info!("Air Density: {} kg / m^3", weather_data.air_density);
            info!("Enthalpy: {} kJ/kg", weather_data.enthalpy);
            info!("wet Bulb: {} C", weather_data.wet_bulb);
            info!("Heat Index: {} F", weather_data.heat_index);
        } else {
            info!("Failed to read sample from sensor");
        }

        /* wait 1 sec before going again */
        Timer::after(Duration::from_secs(1)).await;
    }
}

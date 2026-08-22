/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * PFD
 *
 * FILE:
 * main.rs
 *
 * Description:
 * Blinking LED ensures Wi-Fi connection and data collection
 * Using BME280, MPU6050, QMC5883P sensors and GPS Module
 **************************************************************/

#![no_std]
#![no_main]

/* Crates */
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    dma::InterruptHandler as DmaHandler,
    i2c::InterruptHandler as I2cHandler,
    peripherals::{DMA_CH0, I2C0, PIO0, USB},
    pio::InterruptHandler as PioHanlder,
    usb::{Driver as UsbDriver, InterruptHandler as UsbHandler},
};
use embassy_time::{Duration, Timer};
use gdu::psychometric::SensorData;
use log::info;

/* Tasks */
mod tasks;
use tasks::logger_task::logger_task;

/* Drivers */
mod drivers;
use drivers::{bme280_driver::init_bme280, cyw43_driver::init_cyw43};

use {defmt_rtt as _, panic_probe as _};

/* Interrupt Handlers */
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioHanlder<PIO0>;
    DMA_IRQ_0 => DmaHandler<DMA_CH0>;
    I2C0_IRQ => I2cHandler<I2C0>;
    USBCTRL_IRQ =>UsbHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    /* Init RP2350 peripherals */
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    /* Initialize Logger */
    let driver = UsbDriver::new(p.USB, Irqs);
    spawner.spawn(unwrap!(logger_task(driver)));
    info!("Configured logger");
    info!("PFD start");

    info!("DEBUG Configuring CYW43 Wi-Fi chip");
    /* Configure CYW43 chip */
    init_cyw43(
        spawner, p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0, Irqs,
    )
    .await;

    /* Configure BME 280 sensor */
    info!("BME PERIFERALS");
    let spi = p.SPI0;
    let sck = p.PIN_2;
    let sdi = p.PIN_3;
    let sdo = p.PIN_4;
    let cs = p.PIN_5;
    info!("BEFORE BME INIT");
    let mut bme280 = init_bme280(spi, sck, sdi, sdo, cs);
    info!("AFTER BME INIT");

    /* infinite main loop */
    loop {
        info!("Alive");
        /* read data from single sample */
        let measurements = bme280.measure(&mut embassy_time::Delay).unwrap();

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
            info!("Saturation Vapor Pressure: {} hPa", weather_data.saturation_vapor_pressure);
            info!("Vapor Pressure: {} hPa", weather_data.vapor_pressure);
            info!("Dew Point: {} C", weather_data.dew_point);
            info!("Vapor Pressure Deficit: {} hPa", weather_data.vapor_pressure_deficit);
            info!("Absolute Humidity: {} g/m^3", weather_data.absolute_humidity);
            info!("Mixing Ratio: {} g water / kg dry air", weather_data.mixing_ratio); 
            info!("Specific Humidity: {} g water / kg dry air", weather_data.specific_humidity);
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

/* Metadata */
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"barometer"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico 2 W's onboard LED, connected to GPIO 0 of the cyw43 \
        (WiFi chip) via PIO 0 over the SPI bus."
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file

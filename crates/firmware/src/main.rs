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
use core::cell::RefCell;
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
use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};
use embassy_time::{Duration, Timer};
use gdu::psychometric::SensorData;
use log::info;
use serde_json_core;

/* Tasks */
mod tasks;
use tasks::logger_task::logger_task;

/* Drivers */
mod drivers;
use drivers::{bme280_driver::init_bme280, cyw43_driver::init_cyw43};

/* Constants */
const BUF_SIZE: usize = 1 << 9;

use {defmt_rtt as _, panic_probe as _};

/* Structs */
#[derive(Debug, Clone, Copy)]
struct WeatherPacket {
    data: [u8; BUF_SIZE],
    len: usize,
}

/* Statics */
static WEATHER_DATA: Mutex<CriticalSectionRawMutex, RefCell<Option<WeatherPacket>>> =
    Mutex::new(RefCell::new(None));

/* Interrupt Handlers */
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioHanlder<PIO0>;
    DMA_IRQ_0 => DmaHandler<DMA_CH0>;
    I2C0_IRQ => I2cHandler<I2C0>;
    USBCTRL_IRQ =>UsbHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("PFD start");

    /* Init RP2350 peripherals */
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    /* Initialize Logger */
    let driver = UsbDriver::new(p.USB, Irqs);
    spawner.spawn(unwrap!(logger_task(driver)));

    /* Configure CYW43 chip */
    init_cyw43(
        spawner, p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0, Irqs,
    )
    .await;

    /* Configure BME 280 sensor */
    let mut bme280 = init_bme280(p.I2C0, p.PIN_4, p.PIN_5, Irqs).await;

    /* infinite main loop */
    loop {
        /* read data from single sample */
        let measurements: bme280_rs::Sample = unwrap!(bme280.read_sample().await);

        /* convert sample to readable data */
        let sensor_data: SensorData = SensorData {
            temperature: measurements.temperature,
            pressure: measurements.pressure,
            humidity: measurements.humidity,
        };

        /* compute weather data from sensor data */
        if let Some(weather_data) = sensor_data.calculate() {
            /* serialize to json */
            let mut buf = [0u8; BUF_SIZE];
            match serde_json_core::to_slice(&weather_data, &mut buf) {
                Ok(len) => {
                    let json_bytes = &buf[..len];
                    if let Ok(json_str) = core::str::from_utf8(json_bytes) {
                        info!("JSON: {}", json_str);
                    }

                    let pkt = WeatherPacket { data: buf, len };

                    WEATHER_DATA.lock(|data| {
                        data.borrow_mut().replace(pkt);
                    });
                }
                Err(err) => info!("Serialization failed, error: {:?}", err),
            }
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

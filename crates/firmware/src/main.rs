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

use cortex_m_rt::{exception, ExceptionFrame};
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut};
/* Crates */
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
use log::{error, info};

/* Tasks */
mod tasks;
use tasks::logger_task::logger_task;

/* Drivers */
mod drivers;
use drivers::{bme280_driver::init_bme280, cyw43_driver::init_cyw43};

/* Interrupt Handlers */
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioHanlder<PIO0>;
    DMA_IRQ_0 => DmaHandler<DMA_CH0>;
    I2C0_IRQ => I2cHandler<I2C0>;
    USBCTRL_IRQ =>UsbHandler<USB>;
});

const PANIC_MAGIC: u32 = 0x5041_4E43;

#[repr(C)]
struct PanicRecord { magic: u32, len: u32, msg: [u8; 240] }

// .uninit is not zeroed at boot, so this survives a reset
#[unsafe(link_section = ".uninit.panic_record")]
static mut PANIC_RECORD: MaybeUninit<PanicRecord> = MaybeUninit::uninit();

struct Cursor<'a> { buf: &'a mut [u8], pos: usize }
impl Write for Cursor<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let n = s.len().min(self.buf.len() - self.pos);
        self.buf[self.pos..self.pos + n].copy_from_slice(&s.as_bytes()[..n]);
        self.pos += n;
        Ok(())
    }
}

fn record(args: core::fmt::Arguments) -> ! {
    unsafe {
        let rec = addr_of_mut!(PANIC_RECORD) as *mut PanicRecord;
        let mut c = Cursor { buf: &mut *addr_of_mut!((*rec).msg), pos: 0 };
        let _ = c.write_fmt(args);
        (*rec).len = c.pos as u32;
        (*rec).magic = PANIC_MAGIC;
    }
    cortex_m::peripheral::SCB::sys_reset()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    record(format_args!("{}", info))
}

#[cortex_m_rt::exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    record(format_args!("HardFault pc={:#010x} lr={:#010x}", ef.pc(), ef.lr()))
}

fn report_last_panic() {
    unsafe {
        let rec = addr_of_mut!(PANIC_RECORD) as *mut PanicRecord;
        if core::ptr::read_volatile(addr_of!((*rec).magic)) == PANIC_MAGIC {
            let len = (core::ptr::read_volatile(addr_of!((*rec).len)) as usize).min(240);
            let msg = &*addr_of!((*rec).msg);
            if let Ok(s) = core::str::from_utf8(&msg[..len]) {
                log::error!("LAST PANIC: {}", s);
            }
            core::ptr::write_volatile(addr_of_mut!((*rec).magic), 0);
        }
    }
}

#[embassy_executor::task]
async fn tick() {
    let mut n = 0u32;
    loop {
        log::info!("tick {}", n);
        n += 1;
        embassy_time::Timer::after_secs(1).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("PFD start");

    /* Init RP2350 peripherals */
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    /* Initialize Logger */
    info!("Configure logger");
    let driver = UsbDriver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver).unwrap());
    Timer::after(Duration::from_secs(3)).await;
    report_last_panic();
    spawner.spawn(tick().unwrap());

    info!("DEBUG Configuring CYW43 Wi-Fi chip");
    /* Configure CYW43 chip */
    init_cyw43(
        spawner, p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0, Irqs,
    )
    .await;

    /* Configure BME 280 sensor */
    info!("BME PERIFERALS");
    let spi = p.SPI0;
    let sck = p.PIN_18;
    let sdi = p.PIN_19;
    let sdo = p.PIN_16;
    let cs = p.PIN_17;
    info!("BEFORE BME INIT");
    let mut bme280 = match init_bme280(spi, sck, sdi, sdo, cs) {
        Ok(b) => b,
        Err(_) => loop {
            error!("BME280 init failed, check wiring");
            Timer::after_secs(1).await;
        }
    };
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

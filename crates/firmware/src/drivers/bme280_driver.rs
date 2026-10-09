use bme280::spi::BME280;
use embassy_rp::{
    Peri, 
    peripherals::{SPI0, PIN_16, PIN_17, PIN_18, PIN_19}, 
    spi::{Config, Spi},
    gpio::{Level, Output},
};
use embassy_time::Delay;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use log::{error, info};
use panic_halt as _;

type BME280Spi = ExclusiveDevice<
    Spi<'static, SPI0, embassy_rp::spi::Blocking>,
    Output<'static>,
    NoDelay,
>;

pub fn init_bme280(
    spi_periph: Peri<'static, SPI0>,
    sck: Peri<'static, PIN_18>,
    sdi: Peri<'static, PIN_19>,
    sdo: Peri<'static, PIN_16>,
    cs: Peri<'static, PIN_17>,
) -> Result<BME280<BME280Spi>, ()> {
    info!("INSIDE BME INIT");
    let mut config = Config::default();
    config.frequency = 1_000_000; // 1 MHz

    info!("bme set up spi device and bus");
    let spi_cs = Output::new(cs, Level::High);
    let spi_bus = Spi::new_blocking(spi_periph, sck, sdi, sdo, config);
    let spi_device = ExclusiveDevice::new_no_delay(spi_bus, spi_cs).unwrap();

    let mut bme280 = match BME280::new(spi_device) {
        Ok(b) => b,
        Err(_) => {
            error!("BME280 new() failed");
            return Err(());
        }
    };


    info!("bme initialization");
    let mut delay = Delay;
    if bme280.init(&mut delay).is_err() {
        error!("BME280 init failed - check chip id and wiring");
        return Err(());
    };

    info!("bme init success");
    Ok(bme280)
}

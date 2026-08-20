use bme280::spi::BME280;
use defmt::*;
use embassy_rp::{
    Peri, 
    peripherals::{SPI0, PIN_2, PIN_3, PIN_4, PIN_5}, 
    spi::{Config, Spi},
    gpio::{Level, Output}
};
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;

type BME280Spi = ExclusiveDevice<
    Spi<'static, SPI0, embassy_rp::spi::Blocking>,
    Output<'static>,
    Delay,
>;

pub fn init_bme280(
    spi_periph: Peri<'static, SPI0>,
    sck: Peri<'static, PIN_2>,
    sdi: Peri<'static, PIN_3>,
    sdo: Peri<'static, PIN_4>,
    cs: Peri<'static, PIN_5>,
) -> BME280<BME280Spi> {
    let mut config = Config::default();
    config.frequency = 10_000_000; // 10 MHz

    let cs = Output::new(cs, Level::High);
    let spi_bus = Spi::new_blocking(spi_periph, sck, sdi, sdo, config);
    let spi_device = unwrap!(ExclusiveDevice::new(spi_bus, cs, Delay));

    let mut bme280 = BME280::new(spi_device).unwrap();

    bme280.init(&mut Delay).unwrap();

    bme280
}

use bme280::spi::BME280;
use embassy_rp::{
    Peri,
    gpio::{Level, Output},
    peripherals::{PIN_16, PIN_17, PIN_18, PIN_19, SPI0},
    spi::{Config, Spi},
};
use embassy_time::Delay;
use embedded_hal_1::spi::{ErrorType, Operation, SpiDevice};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use log::{error, info};

pub struct Bme280SpiFix<D>(pub D);

impl<D: SpiDevice> ErrorType for Bme280SpiFix<D> {
    type Error = D::Error;
}

impl<D: SpiDevice> SpiDevice for Bme280SpiFix<D> {
    fn transaction(&mut self, ops: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.0.transaction(ops)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        if write.len() == 1 && !read.is_empty() {
            // register read: address byte, then data bytes
            self.0
                .transaction(&mut [Operation::Write(write), Operation::Read(read)])
        } else {
            // register writes ([reg, value]) work as-is
            self.0.transaction(&mut [Operation::Transfer(read, write)])
        }
    }
}

pub type BME280Spi = Bme280SpiFix<
    ExclusiveDevice<Spi<'static, SPI0, embassy_rp::spi::Blocking>, Output<'static>, NoDelay>,
>;

pub fn init_bme280(
    spi_periph: Peri<'static, SPI0>,
    sck: Peri<'static, PIN_18>,
    sdi: Peri<'static, PIN_19>,
    sdo: Peri<'static, PIN_16>,
    cs: Peri<'static, PIN_17>,
) -> Result<BME280<BME280Spi>, ()> {
    let mut config = Config::default();
    config.frequency = 1_000_000; // 1 MHz

    info!("BME 280 set up spi device and bus");
    let spi_cs = Output::new(cs, Level::High);
    let spi_bus = Spi::new_blocking(spi_periph, sck, sdi, sdo, config);
    let spi_device = ExclusiveDevice::new_no_delay(spi_bus, spi_cs).unwrap();

    let mut bme280 = match BME280::new(Bme280SpiFix(spi_device)) {
        Ok(b) => b,
        Err(e) => {
            error!("BME 280 new() failed: {:?}", e);
            return Err(());
        }
    };

    info!("BME 280 initialization");
    let mut delay = Delay;
    if let Err(e) = bme280.init(&mut delay) {
        error!("BME 280 init failed: {:?}", e);
        return Err(());
    };

    info!("BME 280 init success");
    Ok(bme280)
}

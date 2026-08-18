use bme280_rs::{AsyncBme280, Configuration, Oversampling, SensorMode};
use defmt::*;
use embassy_rp::{
    Peri,
    i2c::{Config, I2c},
    peripherals::{I2C0, PIN_4, PIN_5},
};
use embassy_time::Delay;

pub async fn init_bme280(
    i2c_periph: Peri<'static, I2C0>,
    sda: Peri<'static, PIN_4>,
    scl: Peri<'static, PIN_5>,
    irqs: crate::Irqs,
) -> AsyncBme280<I2c<'static, I2C0, embassy_rp::i2c::Async>, Delay> {
    let i2c = I2c::new_async(i2c_periph, scl, sda, irqs, Config::default());
    let delay = Delay;
    let mut bme280 = AsyncBme280::new(i2c, delay);

    unwrap!(bme280.init().await);

    unwrap!(
        bme280
            .set_sampling_configuration(
                Configuration::default()
                    .with_temperature_oversampling(Oversampling::Oversample1)
                    .with_pressure_oversampling(Oversampling::Oversample1)
                    .with_humidity_oversampling(Oversampling::Oversample1)
                    .with_sensor_mode(SensorMode::Normal)
            )
            .await
    );

    bme280
}

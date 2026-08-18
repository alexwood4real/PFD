use cyw43::{Runner, SpiBus};
use cyw43_pio::PioSpi;
use embassy_rp::{
    gpio::Output,
    peripherals::PIO0
};

#[embassy_executor::task]
pub async fn cyw43_task(
    runner: Runner<'static, SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}
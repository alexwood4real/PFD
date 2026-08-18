use cyw43::{JoinOptions, aligned_bytes};
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_rp::{
    dma::Channel,
    gpio::{Level, Output},
    peripherals::{PIO0, PIN_23, PIN_24, PIN_25, PIN_29, DMA_CH0},
    pio::Pio,
    Peri
};
use static_cell::StaticCell;

/* Tasks */
use crate::tasks::{
    cyw43_task::cyw43_task,
    heartbeat_task::heartbeat_task,
    net_task::net_task,
    tcp_server_task::tcp_server_task
};

/* Constants */
const WIFI_NETWORK: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

pub async fn init_cyw43(
    spawner: Spawner,
    low: Peri<'static, PIN_23>,
    high: Peri<'static, PIN_25>,
    pio0: Peri<'static, PIO0>,
    dio: Peri<'static, PIN_24>,
    clk: Peri<'static, PIN_29>,
    dma: Peri<'static, DMA_CH0>,
    irqs: crate::Irqs
) {
    /* Load Wi-Fi firmware */
    let fw: &cyw43::Aligned<cyw43::A4, [u8]> = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm: &cyw43::Aligned<cyw43::A4, [u8]> = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram: &cyw43::Aligned<cyw43::A4, [u8]> = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    /* Configure GPIO's and PIO/SPI */
    let pwr: Output<'_> = Output::new(low, Level::Low);
    let cs: Output<'_> = Output::new(high, Level::High);
    let mut pio: Pio<'_, PIO0> = Pio::new(pio0, irqs);
    let spi: PioSpi<'_, PIO0, 0> = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        dio,
        clk,
        Channel::new(dma, irqs),
    );

    /* Allocate state driver */
    static STATE: StaticCell<cyw43::State> = StaticCell::new();

    /* Create CYW43 driver */
    let state: &mut cyw43::State = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;

    /* turn on background driver */
    spawner.spawn(unwrap!(cyw43_task(runner)));

    /* set up the control */
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    /* configure network stack */
    let net_config = embassy_net::Config::dhcpv4(Default::default());
    let seed: u64 = 0x0123_4567_89ab_cdef;

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, net_runner) = embassy_net::new(
        net_device,
        net_config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(unwrap!(net_task(net_runner)));

    /* try to join the network until success */
    loop {
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => info!("Failed to join network, status = {:?}", err),
        }
    }

    /* Wi-Fi has been connected, waiting for DHCP */
    stack.wait_config_up().await;

    /* turn on heartbeat */
    spawner.spawn(unwrap!(heartbeat_task(control)));

    /* turn on tcp server */
    spawner.spawn(unwrap!(tcp_server_task(stack)));
}
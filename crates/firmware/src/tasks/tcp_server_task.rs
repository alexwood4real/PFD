use embedded_io_async::Write;
use embassy_net::{IpListenEndpoint, Stack, tcp::TcpSocket};
use embassy_time::{Duration, Timer};
use log::info;

use crate::WEATHER_DATA;

/* Constants */
const PORT: u16 = 8080;

#[embassy_executor::task]
pub async fn tcp_server_task(stack: Stack<'static>) -> ! {
    let mut rx_buffer = [0u8; 1024];
    let mut tx_buffer = [0u8; 1024];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        info!("Waiting for connection");
        if let Err(e) = socket
            .accept(IpListenEndpoint {
                addr: None,
                port: PORT,
            })
            .await
        {
            info!("Accept error: {:?}", e);
            continue;
        }

        info!("client connected");
        let pkt = WEATHER_DATA.lock(|data| *data.borrow());

        if let Some(pkt) = pkt {
            if let Err(e) = socket.write_all(&pkt.data[..pkt.len]).await {
                info!("Write error: {:?}", e);
            }
        } else {
            info!("No weather data available yet");
        }

        /* wait 2 sec before closing socket */
        Timer::after(Duration::from_secs(2)).await;

        socket.close();
    }
}
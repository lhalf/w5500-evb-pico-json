use embassy_net::Stack;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use w5500_json::core::relay::relay;

pub async fn run(stack: Stack<'static>) -> ! {
    let mut rx_buf = [0; 4096];
    let mut tx_buf = [0; 4096];
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut buffer = [0; 4096];

    let mut socket = UdpSocket::new(stack, &mut rx_meta, &mut rx_buf, &mut tx_meta, &mut tx_buf);

    socket.bind(1234).unwrap();

    loop {
        relay(&socket, &mut buffer).await;
    }
}

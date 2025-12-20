use crate::hardware::error::Error;
use crate::hardware::wiznet::Runner;
use embassy_net::udp::UdpSocket;
use embassy_net_wiznet::Device;

pub mod board;
pub mod error;
pub mod network;
pub mod wiznet;

pub async fn init() -> Result<
    (
        UdpSocket<'static>,
        Runner,
        embassy_net::Runner<'static, Device<'static>>,
    ),
    Error,
> {
    let mut board = board::init();

    let seed = board.rng.next_u64();

    let (device, ethernet_runner) = wiznet::init(board).await?;

    let (socket, network_runner) = network::init(device, seed).await?;

    Ok((socket, ethernet_runner, network_runner))
}

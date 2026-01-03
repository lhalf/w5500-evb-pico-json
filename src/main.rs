#![no_std]
#![no_main]

mod hardware;

use crate::hardware::board::Board;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::raw::RawSocket;
use embassy_rp::gpio::Output;
use hardware::error::Error;
use hardware::wiznet;
use panic_probe as _;
use w5500_json::relay::relay;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let (board, led) = hardware::board::init();

    let socket = match setup(&spawner, board).await {
        Ok(socket) => socket,
        Err(_) => setup_error(led),
    };

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        relay(&socket, &mut rx_buffer, &mut tx_buffer).await;
    }
}

async fn setup(spawner: &Spawner, board: Board) -> Result<RawSocket<'static>, Error> {
    let (socket, ethernet_runner, network_runner) = hardware::init(board).await?;

    spawner
        .spawn(ethernet_task(ethernet_runner))
        .map_err(|_| Error::SpawnTask)?;

    spawner
        .spawn(network_task(network_runner))
        .map_err(|_| Error::SpawnTask)?;

    Ok(socket)
}

fn setup_error(mut led: Output<'static>) -> ! {
    led.set_high();
    panic!("setup failed")
}

#[embassy_executor::task]
pub async fn ethernet_task(runner: wiznet::Runner) {
    runner.run().await
}

#[embassy_executor::task]
pub async fn network_task(
    mut runner: embassy_net::Runner<'static, embassy_net_wiznet::Device<'static>>,
) {
    runner.run().await
}

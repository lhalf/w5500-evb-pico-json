#![no_std]
#![no_main]

mod hardware;

use embassy_executor::Spawner;
use embassy_net::udp::UdpSocket;
use hardware::error::Error;
use hardware::wiznet;
use panic_halt as _;
use w5500_json::relay::relay;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let socket = setup(&spawner).await.unwrap();

    let mut buffer = [0; 4096];
    loop {
        relay(&socket, &mut buffer).await;
    }
}

async fn setup(spawner: &Spawner) -> Result<UdpSocket<'static>, Error> {
    let (socket, ethernet_runner, network_runner) = hardware::init().await?;

    spawner
        .spawn(ethernet_task(ethernet_runner))
        .map_err(|_| Error::SpawnTask)?;

    spawner
        .spawn(network_task(network_runner))
        .map_err(|_| Error::SpawnTask)?;

    Ok(socket)
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

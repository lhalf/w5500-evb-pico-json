#![no_std]
#![no_main]

mod board;
mod network;
mod run;
mod tasks;
mod wiznet;

use embassy_executor::Spawner;
use panic_halt as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut board = board::init();
    let seed = board.rng.next_u64();
    let device = wiznet::init(&spawner, board).await;
    let stack = network::init(&spawner, device, seed).await;

    run::run(stack).await;
}

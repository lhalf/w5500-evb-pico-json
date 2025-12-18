#![no_std]
#![no_main]

mod tasks;

use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4};
use embassy_net_wiznet::State;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::spi::{Config as SpiConfig, Spi};
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_halt as _;
use static_cell::StaticCell;

use crate::tasks::ethernet::ethernet_task;
use crate::tasks::net::net_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    // SPI Setup
    let mut spi_cfg = SpiConfig::default();
    spi_cfg.frequency = 50_000_000;
    let (miso, mosi, clk) = (p.PIN_16, p.PIN_19, p.PIN_18);
    let spi = Spi::new(p.SPI0, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, spi_cfg);
    let cs = Output::new(p.PIN_17, Level::High);
    let w5500_int = Input::new(p.PIN_21, Pull::Up);
    let w5500_reset = Output::new(p.PIN_20, Level::High);

    // Wiznet State initialization
    let mac_addr = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
    static STATE: StaticCell<State<8, 8>> = StaticCell::new();
    let state = STATE.init(State::<8, 8>::new());

    let (device, runner) = embassy_net_wiznet::new(
        mac_addr,
        state,
        ExclusiveDevice::new(spi, cs, Delay).unwrap(),
        w5500_int,
        w5500_reset,
    )
    .await
    .unwrap();

    // Spawn the Wiznet W5500 driver task
    spawner.spawn(ethernet_task(runner)).unwrap();

    // STATIC IP CONFIGURATION
    let address = Ipv4Cidr::new(Ipv4Address::new(192, 168, 50, 40), 24);
    let gateway = Some(Ipv4Address::new(192, 168, 50, 1));

    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address,
        gateway,
        dns_servers: Default::default(),
    });

    // Init network stack
    let seed = rng.next_u64();
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        device,
        config, // Using static config instead of DHCP
        RESOURCES.init(StackResources::new()),
        seed,
    );

    // Launch network stack task
    spawner.spawn(net_task(runner)).unwrap();

    // Wait for the stack to be ready
    let _cfg = wait_for_config(stack).await;

    // UDP Echo Loop
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut buf = [0; 4096];

    loop {
        let mut socket = UdpSocket::new(
            stack,
            &mut rx_meta,
            &mut rx_buffer,
            &mut tx_meta,
            &mut tx_buffer,
        );

        if socket.bind(1234).is_ok() {
            loop {
                if let Ok((n, ep)) = socket.recv_from(&mut buf).await {
                    let _ = socket.send_to(&buf[..n], ep).await;
                }
            }
        }
    }
}

async fn wait_for_config(stack: Stack<'static>) -> embassy_net::StaticConfigV4 {
    loop {
        if let Some(config) = stack.config_v4() {
            return config.clone();
        }
        yield_now().await;
    }
}

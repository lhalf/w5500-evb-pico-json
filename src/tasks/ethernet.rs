use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Spi};
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;

#[embassy_executor::task]
pub async fn ethernet_task(
    runner: embassy_net_wiznet::Runner<
        'static,
        embassy_net_wiznet::chip::W5500,
        ExclusiveDevice<Spi<'static, SPI0, Async>, Output<'static>, Delay>,
        Input<'static>,
        Output<'static>,
    >,
) -> ! {
    runner.run().await
}

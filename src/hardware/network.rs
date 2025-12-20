use crate::hardware::error::Error;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4};
use static_cell::StaticCell;

const IP_ADDRESS: Ipv4Cidr = Ipv4Cidr::new(Ipv4Address::new(192, 168, 50, 40), 24);
const PORT: u16 = 1234;
const GATEWAY: Ipv4Address = Ipv4Address::new(192, 168, 50, 1);

pub async fn init(
    device: embassy_net_wiznet::Device<'static>,
    seed: u64,
) -> Result<
    (
        UdpSocket<'static>,
        embassy_net::Runner<'static, embassy_net_wiznet::Device<'static>>,
    ),
    Error,
> {
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: IP_ADDRESS,
        gateway: Some(GATEWAY),
        dns_servers: Default::default(),
    });

    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    Ok((setup_socket(stack)?, runner))
}

fn setup_socket(stack: Stack<'static>) -> Result<UdpSocket<'static>, Error> {
    static RX_BUF: StaticCell<[u8; 4096]> = StaticCell::new();
    static TX_BUF: StaticCell<[u8; 4096]> = StaticCell::new();
    static RX_META: StaticCell<[PacketMetadata; 16]> = StaticCell::new();
    static TX_META: StaticCell<[PacketMetadata; 16]> = StaticCell::new();

    let mut socket = UdpSocket::new(
        stack,
        RX_META.init([PacketMetadata::EMPTY; 16]),
        RX_BUF.init([0; 4096]),
        TX_META.init([PacketMetadata::EMPTY; 16]),
        TX_BUF.init([0; 4096]),
    );

    socket.bind(PORT).map_err(|_| Error::BindPort)?;

    Ok(socket)
}

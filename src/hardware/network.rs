use crate::hardware::error::Error;
use embassy_net::raw::RawSocket;
use embassy_net::raw::{IpProtocol, IpVersion, PacketMetadata};
use embassy_net::{Ipv4Cidr, Stack, StackResources, StaticConfigV4};
use static_cell::StaticCell;
use w5500_json::config::{GATEWAY, IP_ADDRESS, IP_ADDRESS_PREFIX};

pub async fn init(
    device: embassy_net_wiznet::Device<'static>,
    seed: u64,
) -> Result<
    (
        RawSocket<'static>,
        embassy_net::Runner<'static, embassy_net_wiznet::Device<'static>>,
    ),
    Error,
> {
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(IP_ADDRESS, IP_ADDRESS_PREFIX),
        gateway: Some(GATEWAY),
        dns_servers: Default::default(),
    });

    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    Ok((setup_socket(stack)?, runner))
}

fn setup_socket(stack: Stack<'static>) -> Result<RawSocket<'static>, Error> {
    const RX_BUFFER_SIZE: usize = 16 * 1024;
    const TX_BUFFER_SIZE: usize = 16 * 1024;
    const META_SIZE: usize = 256;

    static RX_BUF: StaticCell<[u8; RX_BUFFER_SIZE]> = StaticCell::new();
    static TX_BUF: StaticCell<[u8; TX_BUFFER_SIZE]> = StaticCell::new();
    static RX_META: StaticCell<[PacketMetadata; META_SIZE]> = StaticCell::new();
    static TX_META: StaticCell<[PacketMetadata; META_SIZE]> = StaticCell::new();

    let socket = RawSocket::new::<embassy_net_wiznet::Device<'static>>(
        stack,
        IpVersion::Ipv4,
        IpProtocol::Udp,
        RX_META.init([PacketMetadata::EMPTY; META_SIZE]),
        RX_BUF.init([0; RX_BUFFER_SIZE]),
        TX_META.init([PacketMetadata::EMPTY; META_SIZE]),
        TX_BUF.init([0; TX_BUFFER_SIZE]),
    );

    Ok(socket)
}

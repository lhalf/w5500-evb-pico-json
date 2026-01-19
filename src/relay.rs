use crate::config::{IP_ADDRESS, PORT, RELAY_IP_ADDRESS, RELAY_PORT};
use crate::socket::Socket;
use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::{IpProtocol, Ipv4Packet, Ipv4Repr, UdpPacket, UdpRepr};

const PAYLOAD_OFFSET: usize = 28;

pub async fn relay<'a>(
    socket: &impl Socket<'a>,
    rx_buffer: &'a mut [u8; 4096],
    tx_buffer: &'a mut [u8; 4096],
) {
    if let Ok(data) = socket.recv(rx_buffer).await
        && data.len() > PAYLOAD_OFFSET
    {
        let payload = &data[PAYLOAD_OFFSET..];
        if serde_json_core::from_slice::<serde::de::IgnoredAny>(payload).is_ok() {
            defmt::info!("{:a}", payload);
            let _ = socket.send(tx_packet(tx_buffer, payload)).await;
        }
    }
}

fn tx_packet<'a>(buffer: &'a mut [u8; 4096], payload: &'a [u8]) -> &'a [u8] {
    let udp_repr = UdpRepr {
        src_port: PORT,
        dst_port: RELAY_PORT,
    };

    let ip_repr = Ipv4Repr {
        src_addr: IP_ADDRESS,
        dst_addr: RELAY_IP_ADDRESS,
        next_header: IpProtocol::Udp,
        payload_len: udp_repr.header_len() + payload.len(),
        hop_limit: 64,
    };

    let ip_len = ip_repr.buffer_len();
    let total_len = ip_len + udp_repr.header_len() + payload.len();

    let packet_buffer = &mut buffer[..total_len];

    ip_repr.emit(
        &mut Ipv4Packet::new_unchecked(&mut packet_buffer[..ip_len]),
        &ChecksumCapabilities::default(),
    );

    udp_repr.emit(
        &mut UdpPacket::new_unchecked(&mut packet_buffer[ip_len..]),
        &IP_ADDRESS.into(),
        &RELAY_IP_ADDRESS.into(),
        payload.len(),
        |buf| buf.copy_from_slice(payload),
        &ChecksumCapabilities::default(),
    );

    packet_buffer
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::socket::SocketSpy;
    use alloc::vec;
    use alloc::vec::Vec;
    use embassy_net::raw::RecvError;

    #[tokio::test]
    async fn recv_error_causes_nothing_to_be_relayed() {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        let socket_spy = SocketSpy::default();

        socket_spy
            .recv
            .returns
            .set([Err(RecvError::from(RecvError::Truncated))]);

        relay(&socket_spy, &mut rx_buffer, &mut tx_buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }

    #[tokio::test]
    async fn packet_too_short_causes_nothing_to_be_relayed() {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        let socket_spy = SocketSpy::default();

        socket_spy.recv.returns.set([Ok(&[0u8; 28] as &[u8])]);

        relay(&socket_spy, &mut rx_buffer, &mut tx_buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }

    #[tokio::test]
    async fn valid_json_packets_are_relayed() {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        let rx_packet = create_rx_packet(b"{}");

        let socket_spy = SocketSpy::default();

        socket_spy.recv.returns.set([Ok(rx_packet.as_slice())]);
        socket_spy.send.returns.set([()]);

        relay(&socket_spy, &mut rx_buffer, &mut tx_buffer).await;

        let tx_packet = create_tx_packet(b"{}");

        assert_eq!([tx_packet], socket_spy.send.arguments);
    }

    #[tokio::test]
    async fn invalid_json_packets_are_not_relayed() {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        let rx_packet = create_rx_packet(b"{");

        let socket_spy = SocketSpy::default();

        socket_spy.recv.returns.set([Ok(rx_packet.as_slice())]);

        relay(&socket_spy, &mut rx_buffer, &mut tx_buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }

    fn create_rx_packet(payload: &[u8]) -> Vec<u8> {
        let mut buffer = vec![0u8; PAYLOAD_OFFSET + payload.len()];
        buffer[PAYLOAD_OFFSET..].copy_from_slice(payload);
        buffer
    }

    fn create_tx_packet(payload: &[u8]) -> Vec<u8> {
        let mut buffer = [0; 4096];
        tx_packet(&mut buffer, payload).to_vec()
    }
}

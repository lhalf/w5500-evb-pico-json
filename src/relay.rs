use crate::udp::UdpIO;

pub async fn relay<'a>(socket: &impl UdpIO<'a>, buffer: &'a mut [u8; 4096]) {
    if let Ok((data, metadata)) = socket.recv(buffer).await
        && serde_json_core::from_slice::<serde::de::IgnoredAny>(data).is_ok()
    {
        let _ = socket.send(data, metadata).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::udp::UdpIOSpy;
    use core::net::Ipv4Addr;
    use embassy_net::IpEndpoint;
    use embassy_net::udp::{RecvError, UdpMetadata};

    #[tokio::test]
    async fn packet_too_large_for_buffer_causes_nothing_to_be_sent() {
        let mut buffer = [0; 4096];

        let socket_spy = UdpIOSpy::default();

        socket_spy
            .recv
            .returns
            .set([Err(RecvError::from(RecvError::Truncated))]);

        relay(&socket_spy, &mut buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }

    #[tokio::test]
    async fn valid_json_packets_are_echoed() {
        let mut buffer = [0; 4096];

        let socket_spy = UdpIOSpy::default();

        let metadata = UdpMetadata {
            endpoint: IpEndpoint::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0),
            local_address: None,
            meta: Default::default(),
        };

        let data = b"{}".as_slice();

        socket_spy.recv.returns.set([Ok((data, metadata))]);
        socket_spy.send.returns.set([Ok(())]);

        relay(&socket_spy, &mut buffer).await;

        assert_eq!([(data.to_vec(), metadata)], socket_spy.send.arguments);
    }

    #[tokio::test]
    async fn invalid_json_is_not_echoed() {
        let mut buffer = [0; 4096];

        let socket_spy = UdpIOSpy::default();

        let metadata = UdpMetadata {
            endpoint: IpEndpoint::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0),
            local_address: None,
            meta: Default::default(),
        };

        let data = b"{".as_slice();

        socket_spy.recv.returns.set([Ok((data, metadata))]);

        relay(&socket_spy, &mut buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }
}

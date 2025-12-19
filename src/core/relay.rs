use crate::core::udp::UdpIO;

pub async fn relay(socket: &impl UdpIO, buffer: &mut [u8; 4096]) {
    if let Ok((size, metadata)) = socket.recv(buffer).await {
        let _ = socket.send(&buffer[..size], metadata).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::udp::UdpIOSpy;
    use embassy_net::udp::RecvError;

    #[tokio::test]
    async fn packet_too_large_for_buffer_causes_nothing_to_be_sent() {
        let socket_spy = UdpIOSpy::default();

        socket_spy
            .recv
            .returns
            .set([Err(RecvError::from(RecvError::Truncated))]);

        let mut buffer = [0; 4096];

        relay(&socket_spy, &mut buffer).await;

        assert!(socket_spy.send.arguments.is_empty());
    }
}

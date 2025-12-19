use crate::udp::UdpIO;

pub async fn relay<'a>(socket: &impl UdpIO<'a>, buffer: &'a mut [u8; 4096]) {
    if let Ok((data, metadata)) = socket.recv(buffer).await {
        let _ = socket.send(data, metadata).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::udp::UdpIOSpy;
    use embassy_net::udp::RecvError;

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
}

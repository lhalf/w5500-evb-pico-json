use core::result::Result;
use embassy_net::udp::{RecvError, SendError, UdpMetadata, UdpSocket};

#[cfg_attr(test, autospy::autospy)]
#[allow(async_fn_in_trait)]
pub trait UdpIO {
    async fn recv(&self, buffer: &mut [u8]) -> Result<(usize, UdpMetadata), RecvError>;
    async fn send(&self, data: &[u8], metadata: UdpMetadata) -> Result<(), SendError>;
}

impl UdpIO for UdpSocket<'_> {
    async fn recv(&self, buffer: &mut [u8]) -> Result<(usize, UdpMetadata), RecvError> {
        self.recv_from(buffer).await
    }

    async fn send(&self, data: &[u8], metadata: UdpMetadata) -> Result<(), SendError> {
        self.send_to(data, metadata).await
    }
}

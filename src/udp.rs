use core::result::Result;
use embassy_net::udp::{RecvError, SendError, UdpMetadata, UdpSocket};

#[cfg_attr(test, autospy::autospy)]
#[allow(async_fn_in_trait)]
pub trait UdpIO<'a> {
    async fn recv(&self, buffer: &'a mut [u8]) -> Result<(&'a [u8], UdpMetadata), RecvError>;
    async fn send(&self, data: &[u8], metadata: UdpMetadata) -> Result<(), SendError>;
}

impl<'a> UdpIO<'a> for UdpSocket<'_> {
    async fn recv(&self, buffer: &'a mut [u8]) -> Result<(&'a [u8], UdpMetadata), RecvError> {
        let (size, metadata) = self.recv_from(buffer).await?;
        Ok((&buffer[..size], metadata))
    }

    async fn send(&self, data: &[u8], metadata: UdpMetadata) -> Result<(), SendError> {
        self.send_to(data, metadata).await
    }
}

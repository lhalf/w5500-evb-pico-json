use std::net::UdpSocket;
use std::time::Duration;
use w5500_json::config::{GATEWAY, IP_ADDRESS, PORT};

const TIMEOUT: Duration = Duration::from_secs(1);

#[test]
fn valid_json_is_echoed() {
    let socket = UdpSocket::bind((GATEWAY, 0)).unwrap();

    socket.set_read_timeout(Some(TIMEOUT)).unwrap();

    let payload = b"{}";

    socket.send_to(payload, (IP_ADDRESS, PORT)).unwrap();

    let mut buffer = [0; 512];
    let (len, _) = socket.recv_from(&mut buffer).unwrap();

    assert_eq!(&buffer[..len], payload);
}

#[test]
fn invalid_json_is_dropped() {
    let socket = UdpSocket::bind((GATEWAY, 0)).unwrap();

    socket.set_read_timeout(Some(TIMEOUT)).unwrap();

    let payload = b"{";

    socket.send_to(payload, (IP_ADDRESS, PORT)).unwrap();

    let mut buffer = [0; 512];
    assert_eq!(
        std::io::ErrorKind::WouldBlock,
        socket.recv_from(&mut buffer).unwrap_err().kind()
    );
}

use std::net::UdpSocket;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Duration;
use w5500_json::config::{GATEWAY, IP_ADDRESS, PORT, RELAY_PORT};

const TIMEOUT: Duration = Duration::from_millis(10);
static RECEIVING: AtomicBool = AtomicBool::new(true);

pub struct Connection {
    pub socket: UdpSocket,
    pub receiver: mpsc::Receiver<Vec<u8>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Connection {
    pub fn new() -> Self {
        let socket = UdpSocket::bind((GATEWAY, RELAY_PORT)).unwrap();

        socket.connect((IP_ADDRESS, PORT)).unwrap();
        socket.set_read_timeout(Some(TIMEOUT)).unwrap();

        let socket_clone = socket.try_clone().unwrap();
        let (sender, receiver) = mpsc::channel();

        RECEIVING.store(true, std::sync::atomic::Ordering::Relaxed);
        let handle = std::thread::spawn(move || Self::recv(socket_clone, sender));

        Self {
            socket,
            receiver,
            handle: Some(handle),
        }
    }

    fn recv(socket: UdpSocket, sender: Sender<Vec<u8>>) {
        let mut buffer = [0; 512];
        while RECEIVING.load(std::sync::atomic::Ordering::Relaxed) {
            match socket.recv(&mut buffer) {
                Ok(len) => sender.send(buffer[..len].to_vec()).unwrap(),
                Err(_) => continue,
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        RECEIVING.store(false, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

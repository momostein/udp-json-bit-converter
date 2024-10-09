use crate::Args;
use std::io;
use std::io::ErrorKind;
use std::net::SocketAddr::V4;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs, UdpSocket};
use std::time::Duration;

const BEACON_DATA: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];

pub struct Connections {
    pub socket: UdpSocket,
    pub esp_addr: SocketAddr,
    pub touch_designer_addr: SocketAddr,
}

impl Connections {
    pub fn broadcast_beacon(&self) -> io::Result<usize> {
        self.socket.send_to(BEACON_DATA, self.esp_addr)
    }

    pub fn recv_esp_data(&self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            match self.socket.recv(buf) {
                Ok(data_len) => return Ok(data_len),
                Err(err) => match err.kind() {
                    ErrorKind::TimedOut => {
                        eprintln!("Recv timeout, broadcasting beacon...");
                        self.broadcast_beacon()?;
                    }
                    _ => {
                        return Err(err);
                    }
                },
            }
        }
    }

    pub fn send_touch_designer_bytes(&self, bytes: &[u8]) -> io::Result<usize> {
        self.socket.send_to(bytes, self.touch_designer_addr)
    }
}

fn build_esp_addr(args: &Args) -> io::Result<SocketAddr> {
    let v4 = match &args.esp_addr {
        Some(addr_str) => addr_str
            .parse::<Ipv4Addr>()
            .map_err(|e| io::Error::new(ErrorKind::InvalidInput, e.to_string()))?,
        None => Ipv4Addr::BROADCAST,
    };

    let v4 = SocketAddrV4::new(v4, args.esp_port);

    Ok(V4(v4))
}

pub fn open_connections(args: &Args) -> io::Result<Connections> {
    // let esp_addr = V4(SocketAddrV4::new(Ipv4Addr::new(10,0,0,180), args.esp_port));

    let esp_addr = build_esp_addr(args)?;

    eprintln!("ESP address: {esp_addr}");

    let touch_designer_addr =
        args.touch_designer_addr
            .to_socket_addrs()?
            .next()
            .ok_or(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid touch designer address",
            ))?;
    eprintln!("Touch Designer address: {}", touch_designer_addr);

    let socket = UdpSocket::bind(SocketAddrV4::new(
        Ipv4Addr::UNSPECIFIED,
        args.recv_port.unwrap_or(0),
    ))?;

    socket.set_read_timeout(Some(Duration::from_millis(2000)))?;
    socket.set_broadcast(true)?;

    eprintln!("Socket bound to {}", socket.local_addr()?);

    Ok(Connections {
        esp_addr,
        touch_designer_addr,
        socket,
    })
}

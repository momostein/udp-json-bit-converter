use std::net::UdpSocket;

mod conversion;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:7010")?;

    loop {
        let mut buf = [0; 4096];
        let packet_size = socket.recv(&mut buf)?;

        eprintln!("Recv {packet_size} bytes");

        let packet = conversion::unpack_packet(&buf[..packet_size]);
        let json_bytes = serde_json::to_vec(&packet)?;

        socket.send_to(&json_bytes, "127.0.0.1:7000")?;
    }
}


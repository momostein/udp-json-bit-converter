use std::net::UdpSocket;

const BITS_PER_REGISTER: usize = u8::BITS as usize;
const REGISTERS_PER_BOARD: usize = 3;
const SENSORS_PER_PANEL: usize = 12;
const PANELS_PER_BOARD: usize = 2;

type ShiftRegister = [u8; BITS_PER_REGISTER];
type Panel = [u8; SENSORS_PER_PANEL];
type ShiftRegisterBoard = [Panel; PANELS_PER_BOARD];

const INPUT: &[u8] = &[
    0, 1, 2,
    3, 4, 5,
    7, 8, 9,
    4
];

fn unpack_u8(mut value: u8) -> ShiftRegister {
    let mut result = ShiftRegister::default();
    let mut idx = 0;

    while value > 0 {
        result[idx] = value & 1;
        value >>= 1;
        idx += 1;
    }

    result
}

fn unpack_board(chunk: &[u8]) -> Option<ShiftRegisterBoard> {
    let x: Vec<u8> = chunk.iter().copied().flat_map(unpack_u8).collect();

    Some([
        x[..SENSORS_PER_PANEL].try_into().ok()?,
        x[SENSORS_PER_PANEL..].try_into().ok()?,
    ])
}

fn unpack_packet(packet: &[u8]) -> Vec<Panel> {
    packet.chunks_exact(REGISTERS_PER_BOARD).filter_map(unpack_board).flatten().collect()
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:7010")?;

    loop {
        let mut buf = [0; 4096];
        let packet_size = socket.recv(&mut buf)?;

        eprintln!("Recv {packet_size} bytes");

        let packet = unpack_packet(&buf[..packet_size]);
        let json_bytes = serde_json::to_vec(&packet)?;

        socket.send_to(&json_bytes, "127.0.0.1:7000")?;
    }
}


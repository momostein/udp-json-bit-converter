use clap::Parser;

use bit_conversion::unpack_packet;

mod bit_conversion;
mod connections;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 7010)]
    esp_port: u16,

    #[arg(short, long)]
    converter_port: Option<u16>,

    #[arg(short, long, default_value = "127.0.0.1:7000")]
    touch_designer_addr: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let connections = connections::open_connections(&args)?;

    connections.broadcast_beacon()?;
    loop {
        let mut buf = [0; 4096];
        let packet_size = connections.recv_esp_data(&mut buf)?;

        let panels = unpack_packet(&buf[..packet_size]);
        let json_bytes = serde_json::to_vec(&panels)?;

        connections.send_touch_designer_bytes(&json_bytes)?;
    }
}

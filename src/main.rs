use clap::Parser;

use bit_conversion::unpack_packet;

mod bit_conversion;
mod connections;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long, default_value_t = 3333)]
    esp_port: u16,

    #[arg(short = 'a', long)]
    esp_addr: Option<String>,

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

        let packet_size = match connections.recv_esp_data(&mut buf) {
            Ok(size) => size,
            Err(err) => {
                eprintln!("Error on recv: {}", err);
                continue;
            }
        };

        let panels = unpack_packet(&buf[..packet_size]);
        let json_bytes = serde_json::to_vec(&panels)?;

        // println!("{}", String::from_utf8_lossy(&json_bytes));

        if let Err(err) = connections.send_touch_designer_bytes(&json_bytes) {
            eprintln!("Error on send: {}", err);
        };
    }
}

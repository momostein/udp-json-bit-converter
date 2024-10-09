use std::array::from_fn;

const SENSORS_PER_PANEL: usize = 24;

type Panel = [u8; SENSORS_PER_PANEL];

fn guarantee_size(chunk: &[u8]) -> [u8; 3] {
    chunk.try_into().unwrap()
}

fn unpack_panel(panel_registers: [u8; 3]) -> Panel {
    from_fn(|i| {
        let register = i / u8::BITS as usize;
        let bit = i % u8::BITS as usize;
        (panel_registers[register] & (1 << bit) != 0) as u8
    })
}

pub fn unpack_packet(packet: &[u8]) -> Vec<Panel> {
    packet
        .chunks_exact(3)
        .map(guarantee_size)
        .map(unpack_panel)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_panel() {
        const BOARD: [u8; 3] = [0b0000_1010, 0b1111_0100, 0b0011_1110];

        const EXPECTED: Panel = [
            0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0,
        ];

        assert_eq!(unpack_panel(BOARD), EXPECTED);
    }

    #[test]
    fn test_unpack_packet() {
        const PACKET: &[u8] = &[
            0b0000_1010,
            0b1111_0100,
            0b0011_1110,
            0b0110_1110,
            0b1111_0110,
            0b0111_1110,
        ];

        let expected: Vec<Panel> = vec![
            [
                0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0,
            ],
            [
                0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0,
            ],
        ];

        assert_eq!(unpack_packet(PACKET), expected);
    }
}

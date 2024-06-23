use std::array::from_fn;

const SENSORS_PER_PANEL: usize = 12;

type Panel = [u8; SENSORS_PER_PANEL];

fn unpack_panel_int(panel: u16) -> Panel {
    from_fn(|i| ((panel >> i) & 1) as u8)
}

fn reverse_4_bits(bits: u8) -> u8 {
    let mut out = 0;

    for i in 0..4 {
        out |= ((bits >> i) & 0x01) << (3 - i);
    }

    out
}

fn unpack_board(chunk: [u8; 3]) -> [Panel; 2] {
    let mut panels_int: [u16; 2] = [0, 0];

    for (i, register) in chunk.into_iter().enumerate() {
        let left_bits = register & 0x0F;
        let right_bits = reverse_4_bits((register & 0xF0) >> 4);

        panels_int[0] |= (left_bits as u16) << (i * 4);
        panels_int[1] |= (right_bits as u16) << (i * 4);
    }

    panels_int.map(unpack_panel_int)
}

pub fn guarantee_size(chunk: &[u8]) -> [u8; 3] {
    chunk.try_into().unwrap()
}

pub fn unpack_packet(packet: &[u8]) -> Vec<Panel> {
    packet
        .chunks_exact(3)
        .map(guarantee_size)
        .flat_map(unpack_board)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_board() {
        const BOARD: [u8; 3] = [0b0000_1010, 0b1111_0100, 0b0011_1110];

        const EXPECTED: [Panel; 2] = [
            [0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1],
        ];

        assert_eq!(unpack_board(BOARD), EXPECTED);
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
            [0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1],
            [0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1],
            [0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1],
        ];

        assert_eq!(unpack_packet(PACKET), expected);
    }
}

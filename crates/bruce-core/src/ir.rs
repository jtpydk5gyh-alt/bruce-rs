//! IR protocol logic, hardware-independent and host-tested. Durations are mark/space
//! lengths in microseconds (as the `bruce-hal::Ir` trait uses), starting with a mark.
//!
//! NEC first: the protocol behind most TV remotes.

use alloc::vec::Vec;

pub const HEADER_MARK: u16 = 9000;
pub const HEADER_SPACE: u16 = 4500;
pub const BIT_MARK: u16 = 560;
pub const ZERO_SPACE: u16 = 560;
pub const ONE_SPACE: u16 = 1690;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NecFrame {
    pub address: u8,
    pub command: u8,
}

/// Encode a NEC frame into raw mark/space durations (microseconds), ready for `Ir::send_raw`.
pub fn encode_nec(frame: NecFrame) -> Vec<u16> {
    let mut out = Vec::with_capacity(68);
    out.push(HEADER_MARK);
    out.push(HEADER_SPACE);
    for byte in [frame.address, !frame.address, frame.command, !frame.command] {
        for i in 0..8 {
            out.push(BIT_MARK);
            out.push(if (byte >> i) & 1 == 1 {
                ONE_SPACE
            } else {
                ZERO_SPACE
            });
        }
    }
    out.push(BIT_MARK); // stop mark
    out
}

fn near(value: u16, target: u16) -> bool {
    value.abs_diff(target) <= target / 4 + 100
}

/// Decode raw durations into a NEC frame, tolerant of timing jitter. Returns `None` if the
/// header, any bit, or the address/command inverse checks don't hold.
pub fn decode_nec(durations: &[u16]) -> Option<NecFrame> {
    if durations.len() < 66 || !near(durations[0], HEADER_MARK) || !near(durations[1], HEADER_SPACE)
    {
        return None;
    }
    let mut bytes = [0u8; 4];
    for (byte_i, out_byte) in bytes.iter_mut().enumerate() {
        for bit_i in 0..8 {
            let idx = 2 + (byte_i * 8 + bit_i) * 2;
            if !near(durations[idx], BIT_MARK) {
                return None;
            }
            let bit = if near(durations[idx + 1], ONE_SPACE) {
                1u8
            } else if near(durations[idx + 1], ZERO_SPACE) {
                0
            } else {
                return None;
            };
            *out_byte |= bit << bit_i;
        }
    }
    if bytes[0] != !bytes[1] || bytes[2] != !bytes[3] {
        return None;
    }
    Some(NecFrame {
        address: bytes[0],
        command: bytes[2],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let frame = NecFrame {
            address: 0x04,
            command: 0x08,
        };
        assert_eq!(decode_nec(&encode_nec(frame)), Some(frame));
    }

    #[test]
    fn tolerates_jitter() {
        let frame = NecFrame {
            address: 0xA3,
            command: 0x1C,
        };
        let jittered: Vec<u16> = encode_nec(frame)
            .iter()
            .enumerate()
            .map(|(i, &d)| if i % 2 == 0 { d + 40 } else { d - 40 })
            .collect();
        assert_eq!(decode_nec(&jittered), Some(frame));
    }

    #[test]
    fn rejects_bad_header() {
        let mut d = encode_nec(NecFrame {
            address: 1,
            command: 2,
        });
        d[0] = 1000;
        assert_eq!(decode_nec(&d), None);
    }

    #[test]
    fn rejects_broken_inverse() {
        let mut d = encode_nec(NecFrame {
            address: 0x10,
            command: 0x20,
        });
        d[5] = ONE_SPACE; // flip address bit 1 so the inverse check fails
        assert_eq!(decode_nec(&d), None);
    }
}

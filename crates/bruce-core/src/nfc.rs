//! Flipper `.nfc` device file format, hardware-independent and host-tested. Bruce reads
//! and writes card dumps (UID, ATQA/SAK, Mifare Classic blocks) in this key/value format.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write as _;

/// A parsed NFC card dump. `blocks` keeps each `(index, bytes)` as found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfcCard {
    pub device_type: String,
    pub uid: Vec<u8>,
    pub atqa: Vec<u8>,
    pub sak: Option<u8>,
    pub blocks: Vec<(u16, Vec<u8>)>,
}

fn parse_hex(value: &str) -> Option<Vec<u8>> {
    value
        .split_whitespace()
        .map(|token| u8::from_str_radix(token, 16).ok())
        .collect()
}

/// Parse a `.nfc` file. Returns `None` if the UID is missing or any hex value is malformed.
pub fn parse_nfc(contents: &str) -> Option<NfcCard> {
    let mut device_type = String::new();
    let mut uid = Vec::new();
    let mut atqa = Vec::new();
    let mut sak = None;
    let mut blocks = Vec::new();

    for line in contents.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "Device type" => device_type = value.to_string(),
            "UID" => uid = parse_hex(value)?,
            "ATQA" => atqa = parse_hex(value)?,
            "SAK" => sak = parse_hex(value)?.first().copied(),
            _ => {
                if let Some(index) = key.strip_prefix("Block ") {
                    blocks.push((index.trim().parse().ok()?, parse_hex(value)?));
                }
            }
        }
    }

    if uid.is_empty() {
        return None;
    }
    Some(NfcCard {
        device_type,
        uid,
        atqa,
        sak,
        blocks,
    })
}

fn write_hex(out: &mut String, bytes: &[u8]) {
    for (i, byte) in bytes.iter().enumerate() {
        let _ = write!(out, "{}{byte:02X}", if i == 0 { "" } else { " " });
    }
}

/// Serialize an NFC card dump back to `.nfc` text.
pub fn write_nfc(card: &NfcCard) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Filetype: Flipper NFC device");
    let _ = writeln!(out, "Version: 4");
    if !card.device_type.is_empty() {
        let _ = writeln!(out, "Device type: {}", card.device_type);
    }
    out.push_str("UID: ");
    write_hex(&mut out, &card.uid);
    out.push('\n');
    if !card.atqa.is_empty() {
        out.push_str("ATQA: ");
        write_hex(&mut out, &card.atqa);
        out.push('\n');
    }
    if let Some(sak) = card.sak {
        let _ = writeln!(out, "SAK: {sak:02X}");
    }
    for (index, bytes) in &card.blocks {
        let _ = write!(out, "Block {index}: ");
        write_hex(&mut out, bytes);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Filetype: Flipper NFC device\n\
        Version: 4\n\
        Device type: Mifare Classic\n\
        UID: 04 A2 B3 C4\n\
        ATQA: 00 04\n\
        SAK: 08\n\
        Block 0: 04 A2 B3 C4 59 08 04 00 62 63 64 65 66 67 68 69\n";

    #[test]
    fn parses_sample() {
        let card = parse_nfc(SAMPLE).expect("valid .nfc");
        assert_eq!(card.device_type, "Mifare Classic");
        assert_eq!(card.uid, [0x04, 0xA2, 0xB3, 0xC4]);
        assert_eq!(card.atqa, [0x00, 0x04]);
        assert_eq!(card.sak, Some(0x08));
        assert_eq!(card.blocks.len(), 1);
        assert_eq!(card.blocks[0].0, 0);
        assert_eq!(card.blocks[0].1.len(), 16);
    }

    #[test]
    fn round_trip() {
        let card = NfcCard {
            device_type: "Mifare Classic".to_string(),
            uid: alloc::vec![0xDE, 0xAD, 0xBE, 0xEF],
            atqa: alloc::vec![0x00, 0x04],
            sak: Some(0x08),
            blocks: alloc::vec![(1, alloc::vec![0x11, 0x22, 0x33])],
        };
        assert_eq!(parse_nfc(&write_nfc(&card)), Some(card));
    }

    #[test]
    fn rejects_missing_uid() {
        assert_eq!(parse_nfc("Device type: Mifare Classic\nSAK: 08\n"), None);
    }

    #[test]
    fn rejects_bad_hex() {
        assert_eq!(parse_nfc("UID: 04 ZZ B3\n"), None);
    }
}

//! Flipper `.sub` sub-GHz file format, hardware-independent and host-tested. Bruce records
//! and replays RF captures in this key/value text format; here we parse and write it.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write as _;

/// A parsed `.sub` file. `raw_data` holds signed timings in microseconds (+ mark, − space).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubFile {
    pub frequency: u32,
    pub preset: String,
    pub protocol: String,
    pub raw_data: Vec<i32>,
}

/// Parse a `.sub` file. `RAW_Data` may span several lines; their values are concatenated.
/// Returns `None` if `Frequency` is missing or any value fails to parse.
pub fn parse_sub(contents: &str) -> Option<SubFile> {
    let mut frequency = None;
    let mut preset = String::new();
    let mut protocol = String::new();
    let mut raw_data = Vec::new();

    for line in contents.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim();
        match key.trim() {
            "Frequency" => frequency = value.parse().ok(),
            "Preset" => preset = value.to_string(),
            "Protocol" => protocol = value.to_string(),
            "RAW_Data" => {
                for token in value.split_whitespace() {
                    raw_data.push(token.parse().ok()?);
                }
            }
            _ => {}
        }
    }

    Some(SubFile {
        frequency: frequency?,
        preset,
        protocol,
        raw_data,
    })
}

/// Serialize a `.sub` file back to text (one `RAW_Data` line).
pub fn write_sub(file: &SubFile) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Filetype: Flipper SubGhz RAW File");
    let _ = writeln!(out, "Version: 1");
    let _ = writeln!(out, "Frequency: {}", file.frequency);
    if !file.preset.is_empty() {
        let _ = writeln!(out, "Preset: {}", file.preset);
    }
    if !file.protocol.is_empty() {
        let _ = writeln!(out, "Protocol: {}", file.protocol);
    }
    if !file.raw_data.is_empty() {
        out.push_str("RAW_Data:");
        for value in &file.raw_data {
            let _ = write!(out, " {value}");
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Filetype: Flipper SubGhz RAW File\n\
        Version: 1\n\
        Frequency: 433920000\n\
        Preset: FuriHalSubGhzPresetOok650Async\n\
        Protocol: RAW\n\
        RAW_Data: 320 -160 480 -320 160 -160\n";

    #[test]
    fn parses_sample() {
        let f = parse_sub(SAMPLE).expect("valid .sub");
        assert_eq!(f.frequency, 433_920_000);
        assert_eq!(f.protocol, "RAW");
        assert_eq!(f.preset, "FuriHalSubGhzPresetOok650Async");
        assert_eq!(f.raw_data, [320, -160, 480, -320, 160, -160]);
    }

    #[test]
    fn concatenates_multiline_raw_data() {
        let s = "Frequency: 868000000\nRAW_Data: 100 -100\nRAW_Data: 200 -200\n";
        let f = parse_sub(s).expect("valid .sub");
        assert_eq!(f.raw_data, [100, -100, 200, -200]);
    }

    #[test]
    fn round_trip() {
        let f = SubFile {
            frequency: 315_000_000,
            preset: "FuriHalSubGhzPresetOok270Async".to_string(),
            protocol: "RAW".to_string(),
            raw_data: alloc::vec![500, -250, 500, -250],
        };
        assert_eq!(parse_sub(&write_sub(&f)), Some(f));
    }

    #[test]
    fn rejects_missing_frequency() {
        assert_eq!(parse_sub("Protocol: RAW\nRAW_Data: 10 -10\n"), None);
    }

    #[test]
    fn rejects_bad_raw_token() {
        assert_eq!(
            parse_sub("Frequency: 433920000\nRAW_Data: 10 oops -10\n"),
            None
        );
    }
}

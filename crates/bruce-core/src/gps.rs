//! NMEA 0183 parsing, hardware-independent and host-tested. A GPS module streams ASCII
//! sentences like `$GPGGA,...*47`; here we validate the checksum and pull out a fix.

/// A position fix from a GGA sentence. Coordinates are decimal degrees (+N/-S, +E/-W).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GgaFix {
    pub latitude: f64,
    pub longitude: f64,
    pub fix_quality: u8,
    pub satellites: u8,
}

impl GgaFix {
    pub fn has_fix(&self) -> bool {
        self.fix_quality > 0
    }
}

/// Verify an NMEA checksum: XOR of every byte between `$` and `*` equals the trailing hex.
pub fn checksum_ok(sentence: &str) -> bool {
    let s = sentence.trim();
    let s = s.strip_prefix('$').unwrap_or(s);
    let Some((body, rest)) = s.split_once('*') else {
        return false;
    };
    let Some(hex) = rest.get(..2) else {
        return false;
    };
    let Ok(expected) = u8::from_str_radix(hex, 16) else {
        return false;
    };
    body.bytes().fold(0u8, |acc, b| acc ^ b) == expected
}

fn parse_coord(field: &str, hemisphere: &str, degree_digits: usize) -> Option<f64> {
    if field.len() < degree_digits {
        return None;
    }
    let (degrees, minutes) = field.split_at(degree_digits);
    let mut value = degrees.parse::<f64>().ok()? + minutes.parse::<f64>().ok()? / 60.0;
    if hemisphere == "S" || hemisphere == "W" {
        value = -value;
    }
    Some(value)
}

/// Parse a GGA sentence (`$GPGGA`/`$GNGGA`) into a fix. Returns `None` if the checksum
/// fails, the sentence isn't GGA, or a needed field is missing or malformed.
pub fn parse_gga(sentence: &str) -> Option<GgaFix> {
    if !checksum_ok(sentence) {
        return None;
    }
    let body = sentence.trim().strip_prefix('$')?.split_once('*')?.0;
    let mut fields = body.split(',');

    if !fields.next()?.ends_with("GGA") {
        return None;
    }
    let _time = fields.next()?;
    let latitude = parse_coord(fields.next()?, fields.next()?, 2)?;
    let longitude = parse_coord(fields.next()?, fields.next()?, 3)?;
    let fix_quality = fields.next()?.parse().ok()?;
    let satellites = fields.next()?.parse().ok()?;

    Some(GgaFix {
        latitude,
        longitude,
        fix_quality,
        satellites,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";

    #[test]
    fn parses_sample_fix() {
        let fix = parse_gga(SAMPLE).expect("valid GGA");
        assert!((fix.latitude - 48.1173).abs() < 1e-3);
        assert!((fix.longitude - 11.5167).abs() < 1e-3);
        assert_eq!(fix.fix_quality, 1);
        assert_eq!(fix.satellites, 8);
        assert!(fix.has_fix());
    }

    #[test]
    fn rejects_corrupt_checksum() {
        let bad = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
        assert!(!checksum_ok(bad));
        assert_eq!(parse_gga(bad), None);
    }

    #[test]
    fn rejects_non_gga() {
        let rmc = "$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A";
        assert_eq!(parse_gga(rmc), None);
    }

    #[test]
    fn southern_western_hemispheres_are_negative() {
        let s = "$GPGGA,123519,3357.000,S,01810.000,W,1,06,0.9,10.0,M,0.0,M,,*44";
        let fix = parse_gga(s).expect("valid GGA");
        assert!(fix.latitude < 0.0);
        assert!(fix.longitude < 0.0);
    }
}

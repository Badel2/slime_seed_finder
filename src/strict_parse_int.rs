// On one hand, I don't want to write my own parser.
// On the other hand, I don't want to import a library just for this...
use std::convert::TryFrom;

/// Function to parse integers that basically only accepts integers
/// that were formatted using a simple `x.to_string()`.
///
/// Valid examples: "-1", "0", "1"
///
/// Invalid examples:
/// * Empty string: ""
/// * Only negative sign: "-"
/// * Negative zero: "-0"
/// * Positive sign: "+0", "+1"
/// * Leading zeros: "00", "01"
/// * Leading whitespace or any other character: " 0"
/// * Trailing whitespace or any other character: "0 "
/// * Hexadecimal or other bases: "0x0"
/// * Floating point syntax: dots or exponents: "0.", "0.0", "0e0"
pub fn strict_parse_i32(s: &[u8]) -> Option<i32> {
    let negative = *s.get(0)? == b'-';

    if negative {
        // Parse positive value
        let a = strict_parse_u32(&s[1..])?;
        // Two's complement magic
        let x = (!a).wrapping_add(1);
        // Cast u32 to i32, will result in a negative number if the number is valid
        let x = x as i32;
        if x >= 0 {
            // Handle overflow and "-0"
            None
        } else {
            Some(x)
        }
    } else {
        i32::try_from(strict_parse_u32(s)?).ok()
    }
}

/// Function to parse integers that basically only accepts integers
/// that were formatted using a simple `x.to_string()`.
///
/// Valid examples: "0", "1", "2"
///
/// Invalid examples:
/// * Empty string: ""
/// * Only negative sign: "-"
/// * Negative zero: "-0"
/// * Positive sign: "+0", "+1"
/// * Leading zeros: "00", "01"
/// * Leading whitespace or any other character: " 0"
/// * Trailing whitespace or any other character: "0 "
/// * Hexadecimal or other bases: "0x0"
/// * Floating point syntax: dots or exponents: "0.", "0.0", "0e0"
pub fn strict_parse_u32(s: &[u8]) -> Option<u32> {
    if s.len() == 0 {
        // Empty string
        return None;
    }

    // If the first digit is 0, the string cannot have more digits
    if s[0] == b'0' {
        if s.len() > 1 {
            // Leading zero
            return None;
        } else {
            return Some(0);
        }
    }

    let mut r: u32 = 0;

    for c in s {
        if c.is_ascii_digit() {
            // TODO: we can skip overflow checks for the first 9 digits
            r = r.checked_mul(10)?;
            // Convert digit to numerical value
            let x = u32::from(*c - b'0');
            r = r.checked_add(x)?;
        } else {
            // Invalid character
            return None;
        }
    }

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_parse_u32() {
        assert_eq!(strict_parse_u32(b"0"), Some(0));
        assert_eq!(strict_parse_u32(b"1"), Some(1));
        assert_eq!(strict_parse_u32(b"1000000000"), Some(1000000000));
        assert_eq!(
            strict_parse_u32(b"2147483646"),
            Some(u32::try_from(i32::MAX).unwrap() - 1)
        );
        assert_eq!(
            strict_parse_u32(b"2147483647"),
            Some(u32::try_from(i32::MAX).unwrap())
        );
        assert_eq!(
            strict_parse_u32(b"2147483648"),
            Some(u32::try_from(i32::MAX).unwrap() + 1)
        );
        assert_eq!(strict_parse_u32(b"4294967294"), Some(u32::MAX - 1));
        assert_eq!(strict_parse_u32(b"4294967295"), Some(u32::MAX));

        assert_eq!(strict_parse_u32(b""), None);
        assert_eq!(strict_parse_u32(b"-"), None);
        assert_eq!(strict_parse_u32(b"-0"), None);
        assert_eq!(strict_parse_u32(b"+0"), None);
        assert_eq!(strict_parse_u32(b"+1"), None);
        assert_eq!(strict_parse_u32(b"00"), None);
        assert_eq!(strict_parse_u32(b"01"), None);
        assert_eq!(strict_parse_u32(b" 0"), None);
        assert_eq!(strict_parse_u32(b"0 "), None);
        assert_eq!(strict_parse_u32(b" -1"), None);
        assert_eq!(strict_parse_u32(b"- 1"), None);
        assert_eq!(strict_parse_u32(b"1 1"), None);
        assert_eq!(strict_parse_u32(b"0x0"), None);
        assert_eq!(strict_parse_u32(b"0xA"), None);
        assert_eq!(strict_parse_u32(b"0xa"), None);
        assert_eq!(strict_parse_u32(b"A"), None);
        assert_eq!(strict_parse_u32(b"a"), None);
        assert_eq!(strict_parse_u32(b"0o0"), None);
        assert_eq!(strict_parse_u32(b"4294967296"), None);
        assert_eq!(strict_parse_u32(b"-2147483649"), None);
        assert_eq!(strict_parse_u32(b"10000000000"), None);
        assert_eq!(strict_parse_u32(b"-10000000000"), None);
        assert_eq!(strict_parse_u32(b"0."), None);
        assert_eq!(strict_parse_u32(b"0.0"), None);
        assert_eq!(strict_parse_u32(b"0e0"), None);
    }
    #[test]
    fn test_strict_parse_i32() {
        assert_eq!(strict_parse_i32(b"0"), Some(0));
        assert_eq!(strict_parse_i32(b"1"), Some(1));
        assert_eq!(strict_parse_i32(b"-1"), Some(-1));
        assert_eq!(strict_parse_i32(b"2147483647"), Some(i32::MAX));
        assert_eq!(strict_parse_i32(b"-2147483648"), Some(i32::MIN));
        assert_eq!(strict_parse_i32(b"2147483646"), Some(i32::MAX - 1));
        assert_eq!(strict_parse_i32(b"-2147483647"), Some(i32::MIN + 1));

        assert_eq!(strict_parse_i32(b""), None);
        assert_eq!(strict_parse_i32(b"-"), None);
        assert_eq!(strict_parse_i32(b"-0"), None);
        assert_eq!(strict_parse_i32(b"+0"), None);
        assert_eq!(strict_parse_i32(b"+1"), None);
        assert_eq!(strict_parse_i32(b"00"), None);
        assert_eq!(strict_parse_i32(b"01"), None);
        assert_eq!(strict_parse_i32(b" 0"), None);
        assert_eq!(strict_parse_i32(b"0 "), None);
        assert_eq!(strict_parse_i32(b" -1"), None);
        assert_eq!(strict_parse_i32(b"- 1"), None);
        assert_eq!(strict_parse_i32(b"1 1"), None);
        assert_eq!(strict_parse_i32(b"0x0"), None);
        assert_eq!(strict_parse_i32(b"0xA"), None);
        assert_eq!(strict_parse_i32(b"0xa"), None);
        assert_eq!(strict_parse_i32(b"A"), None);
        assert_eq!(strict_parse_i32(b"a"), None);
        assert_eq!(strict_parse_i32(b"0o0"), None);
        assert_eq!(strict_parse_i32(b"2147483648"), None);
        assert_eq!(strict_parse_i32(b"-2147483649"), None);
        assert_eq!(strict_parse_i32(b"10000000000"), None);
        assert_eq!(strict_parse_i32(b"-10000000000"), None);
        assert_eq!(strict_parse_i32(b"0."), None);
        assert_eq!(strict_parse_i32(b"0.0"), None);
        assert_eq!(strict_parse_i32(b"0e0"), None);
    }
}

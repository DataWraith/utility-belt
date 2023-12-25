use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::{map_res, opt},
    sequence::Tuple,
    IResult, Parser,
};

/// nom parser for a usize
pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, num) = nom::character::complete::digit1(input)?;
    let num = num.parse::<usize>().unwrap();

    Ok((input, num))
}

/// nom parser for a isize
pub fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (input, sign) = opt(tag("-"))(input)?;
    let (input, num) = nom::character::complete::digit1(input)?;
    let num = num.parse::<isize>().unwrap();

    if sign.is_some() {
        Ok((input, -num))
    } else {
        Ok((input, num))
    }
}

/// nom parser for an RGB color specified in the usual hexadecimal format (e.g. #ff00ff).
///
/// This is adapted from the nom example.
pub fn parse_hex_color(input: &str) -> IResult<&str, (u8, u8, u8)> {
    fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    fn hex_primary(input: &str) -> IResult<&str, u8> {
        map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
    }

    let (input, _) = tag("#")(input)?;

    (hex_primary, hex_primary, hex_primary).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_usize() {
        assert_eq!(parse_usize("123"), Ok(("", 123)));
        assert_eq!(parse_usize("123abc"), Ok(("abc", 123)));
        assert!(parse_usize("abc").is_err());
    }

    #[test]
    fn test_parse_isize() {
        assert_eq!(parse_isize("123"), Ok(("", 123)));
        assert_eq!(parse_isize("-123"), Ok(("", -123)));
        assert_eq!(parse_isize("123abc"), Ok(("abc", 123)));
        assert_eq!(parse_isize("-123abc"), Ok(("abc", -123)));
        assert!(parse_isize("abc").is_err());
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_hex_color("#ff00ff"), Ok(("", (255, 0, 255))));
        assert_eq!(parse_hex_color("#ff00ffabc"), Ok(("abc", (255, 0, 255))));

        // Missing octothorpe
        assert!(parse_hex_color("ff00ff").is_err());
    }
}

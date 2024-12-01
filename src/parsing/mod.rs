pub use winnow::{
    ascii::digit1,
    combinator::{opt, preceded, repeat, separated},
    token::{none_of, take_till},
    PResult, Parser,
};

use winnow::token::any;

fn parse_int(input: &mut &str) -> PResult<i64> {
    (preceded(opt('-'), digit1))
        .take()
        .parse_to()
        .parse_next(input)
}

pub fn parse_ints(input: &str) -> Vec<i64> {
    separated(
        1..,
        parse_int,
        (
            any,
            take_till(0.., |c: char| c.is_ascii_digit() || c == '-'),
        ),
    )
    .parse(input)
    .expect("Couldn't parse ints")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ints() {
        assert_eq!(parse_ints("123"), vec![123]);
        assert_eq!(parse_ints("123 456"), vec![123, 456]);
        assert_eq!(parse_ints("123 456.789"), vec![123, 456, 789]);
        assert_eq!(parse_ints("123 456 -789"), vec![123, 456, -789]);
        assert_eq!(parse_ints("-123@456,-789"), vec![-123, 456, -789]);
        assert_eq!(parse_ints("123--456--789"), vec![123, -456, -789]);
        assert_eq!(parse_ints("123   456\n789"), vec![123, 456, 789]);
    }
}

use itertools::Itertools;

pub fn parse_ints(input: &str) -> Vec<i64> {
    input.split(|c: char| !c.is_ascii_digit() && c != '-').filter(|w| !w.is_empty()).map(|w| w.parse::<i64>().unwrap()).collect_vec()
}

pub fn parse_uints(input: &str) -> Vec<u64> {
    input.split(|c: char| !c.is_ascii_digit()).filter(|w| !w.is_empty()).map(|w| w.parse::<u64>().unwrap()).collect_vec()
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
        assert_eq!(parse_ints("123x-456x-789"), vec![123, -456, -789]);
        assert_eq!(parse_ints("123   456\n789"), vec![123, 456, 789]);
        assert_eq!(parse_ints("123   456\n789\n\n"), vec![123, 456, 789]);
    }

    #[test]
    fn test_parse_uints() {
        assert_eq!(parse_uints("123"), vec![123]);
        assert_eq!(parse_uints("123 456"), vec![123, 456]);
        assert_eq!(parse_uints("123 456.789"), vec![123, 456, 789]);
        assert_eq!(parse_uints("123 456 -789"), vec![123, 456, 789]);
        assert_eq!(parse_uints("-123@456,-789"), vec![123, 456, 789]);
        assert_eq!(parse_uints("123x-456x-789"), vec![123, 456, 789]);
        assert_eq!(parse_uints("Point { x: 123, y: 456 }\n"), vec![123, 456]);
        assert_eq!(parse_uints("123   456\n789\n\n"), vec![123, 456, 789]);
    }
}

// Parses all integers in the input string, regardless of other characters.
pub fn parse_ints(input: &str) -> Vec<i64> {
    input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<i64>().unwrap())
        .collect()
}

// Parses all unsigned integers in the input string, regardless of other characters.
pub fn parse_uints(input: &str) -> Vec<u64> {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<u64>().unwrap())
        .collect()
}

// Parses all uppercase strings in the input string, regardless of other characters.
pub fn parse_capitals(input: &str) -> Vec<String> {
    input
        .split(|c: char| !c.is_ascii_uppercase())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect()
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

    #[test]
    fn test_parse_capitals() {
        assert_eq!(
            parse_capitals("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            vec!["V", "AA", "DD", "II", "BB"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );
    }
}

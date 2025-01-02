use std::str::FromStr;

/// A flexible parsing function that can handle any type implementing FromStr
pub fn parse_values<T: FromStr>(input: &str, is_delimiter: impl Fn(char) -> bool) -> Vec<T> {
    input
        .split(is_delimiter)
        .filter(|w| !w.is_empty())
        .filter_map(|w| w.parse().ok())
        .collect()
}

pub fn parse_ints(input: &str) -> Vec<i64> {
    parse_values(input, |c| !c.is_ascii_digit() && c != '-')
}

pub fn parse_uints(input: &str) -> Vec<u64> {
    parse_values(input, |c| !c.is_ascii_digit())
}

pub fn parse_capitals(input: &str) -> Vec<String> {
    parse_values(input, |c| !c.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_values() {
        // Parse floats
        let floats: Vec<f64> = parse_values("1.5 2.7 3.14", char::is_whitespace);
        assert_eq!(floats, vec![1.5, 2.7, 3.14]);

        // Parse words
        let words: Vec<String> = parse_values::<String>("hello,world", |c| c == ',');
        assert_eq!(words, vec!["hello", "world"]);
    }

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
        assert_eq!(parse_ints("12,34 -> 56,78"), vec![12, 34, 56, 78]);
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

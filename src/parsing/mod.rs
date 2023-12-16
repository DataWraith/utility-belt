use nom::IResult;

/// nom parser for a usize
pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, num) = nom::character::complete::digit1(input)?;
    let num = num.parse::<usize>().unwrap();

    Ok((input, num))
}

/// nom parser for a isize
pub fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (input, num) = nom::character::complete::digit1(input)?;
    let num = num.parse::<isize>().unwrap();

    Ok((input, num))
}

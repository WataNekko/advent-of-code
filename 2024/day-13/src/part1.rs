use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<((i64, i64), (i64, i64), (i64, i64))>> {
    separated_list1(
        line_ending,
        tuple((
            tuple((
                preceded(tag("Button A: X+"), complete::i64),
                delimited(tag(", Y+"), complete::i64, line_ending),
            )),
            tuple((
                preceded(tag("Button B: X+"), complete::i64),
                delimited(tag(", Y+"), complete::i64, line_ending),
            )),
            tuple((
                preceded(tag("Prize: X="), complete::i64),
                delimited(tag(", Y="), complete::i64, line_ending),
            )),
        )),
    )(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64> {
    let (_, input) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let total = input
        .into_iter()
        .map(|(a, b, c)| {
            let determinant = a.0 * b.1 - a.1 * b.0;
            ((a, b, c), determinant)
        })
        .filter(|(_, determinant)| *determinant != 0)
        .map(|((a, b, c), d)| {
            let dx = c.0 * b.1 - c.1 * b.0;
            let dy = a.0 * c.1 - a.1 * c.0;
            (dx, dy, d)
        })
        .filter_map(|(dx, dy, d)| match (dx % d, dy % d) {
            (0, 0) => Some((dx / d, dy / d)),
            _ => None,
        })
        .filter(|(a, b)| 0 <= *a && *a <= 100 && 0 <= *b && *b <= 100)
        // .inspect(|a| _ = dbg!(a))
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        assert_eq!(480, process(input)?);
        Ok(())
    }
}

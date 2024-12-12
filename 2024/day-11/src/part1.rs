use miette::miette;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (_, stones) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let count = (0..25)
        .fold(stones, |stones, _| {
            stones
                .iter()
                .flat_map(|&stone| match stone {
                    0 => [Some(1), None],
                    n => {
                        let s = n.to_string();
                        if s.len() % 2 == 0 {
                            let (a, b) = s.split_at(s.len() / 2);
                            [a.parse().ok(), b.parse().ok()]
                        } else {
                            [Some(n * 2024), None]
                        }
                    }
                })
                .flatten()
                .collect()
        })
        .len();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "125 17";
        assert_eq!(55312, process(input)?);
        Ok(())
    }
}

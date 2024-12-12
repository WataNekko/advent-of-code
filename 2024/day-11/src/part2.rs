use std::collections::HashMap;

use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};
use num_traits::Euclid;

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

#[tracing::instrument]
pub fn process(input: &str, blinks: u32) -> miette::Result<usize> {
    let (_, stones) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let stones = stones.iter().copied().counts();

    let count = (0..blinks)
        .fold(stones, |prev, _| {
            let mut new = HashMap::new();
            prev.iter().for_each(|(&num, &count)| match num {
                0 => {
                    new.entry(1).and_modify(|v| *v += count).or_insert(count);
                }
                n => {
                    let digit_count = n.checked_ilog10().unwrap_or(0) + 1;
                    if digit_count % 2 == 0 {
                        let (a, b) = n.div_rem_euclid(&10u64.pow(digit_count / 2));
                        for n in [a, b] {
                            new.entry(n).and_modify(|v| *v += count).or_insert(count);
                        }
                    } else {
                        new.entry(n * 2024)
                            .and_modify(|v| *v += count)
                            .or_insert(count);
                    }
                }
            });
            new
        })
        .values()
        .sum();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "125 17";
        assert_eq!(55312, process(input, 25)?);
        Ok(())
    }
}

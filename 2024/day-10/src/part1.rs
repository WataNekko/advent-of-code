use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<IVec2>) {
    let mut trailheads = Vec::new();
    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .enumerate()
                    .inspect(|&(x, height)| {
                        if height == 0 {
                            trailheads.push(IVec2::new(x as _, y as _));
                        }
                    })
                    .map(|(_, height)| height)
                    .collect_vec()
            })
            .collect_vec(),
        trailheads,
    )
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (map, trailheads) = parse(input);

    let count = trailheads
        .iter()
        .map(|&trailhead| {
            (1..=9).fold(HashSet::from([trailhead]), |trails, height| {
                trails
                    .iter()
                    .flat_map(|pos| {
                        [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                            .iter()
                            .map(move |dir| pos + dir)
                            .filter(|&IVec2 { x, y }| {
                                match (usize::try_from(x), usize::try_from(y)) {
                                    (Ok(x), Ok(y)) => map
                                        .get(y)
                                        .and_then(|r| r.get(x))
                                        .is_some_and(|&d| d == height),
                                    _ => false,
                                }
                            })
                    })
                    .collect()
            })
        })
        .map(|trail| trail.len())
        .sum();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        assert_eq!(36, process(input)?);
        Ok(())
    }
}

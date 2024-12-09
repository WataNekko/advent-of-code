use std::{collections::HashMap, iter::successors};

use glam::IVec2;
use itertools::Itertools;

fn parse(input: &str) -> (HashMap<char, Vec<IVec2>>, IVec2) {
    let mut bound = IVec2::ZERO;
    let mut antenna_groups: HashMap<_, Vec<_>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        bound.y = bound.y.max(y as _);
        for (x, ch) in line.chars().enumerate() {
            bound.x = bound.x.max(x as _);

            if ch.is_alphanumeric() {
                antenna_groups
                    .entry(ch)
                    .or_default()
                    .push(IVec2::new(x as _, y as _));
            }
        }
    }

    (antenna_groups, bound)
}

fn is_within_bound(pos: IVec2, bound: IVec2) -> bool {
    IVec2::ZERO.cmple(pos).all() && pos.cmple(bound).all()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (antenna_groups, bound) = parse(input);

    let count = antenna_groups
        .values()
        .flat_map(|pos_group| {
            pos_group.iter().copied().permutations(2).flat_map(|pair| {
                let &[a, b] = pair.as_slice() else {
                    panic!("expect permutation of 2");
                };

                let move_vec = a - b;
                successors(Some(a), move |&prev| {
                    Some(prev + move_vec).filter(|&pos| is_within_bound(pos, bound))
                })
            })
        })
        .unique()
        .count();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(34, process(input)?);
        Ok(())
    }
}

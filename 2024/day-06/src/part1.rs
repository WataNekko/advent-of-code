use std::collections::HashSet;

use glam::IVec2;

#[derive(Debug)]
struct Transform {
    pos: IVec2,
    rot: IVec2,
}

#[derive(Debug)]
struct Scene {
    wall_positions: HashSet<IVec2>,
    guard: Transform,
    room_bound: IVec2,
}

fn parse(input: &str) -> Scene {
    let mut wall_positions = HashSet::new();
    let mut guard = None;
    let mut room_bound = IVec2::ZERO;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    wall_positions.insert(IVec2::new(x as _, y as _));
                }
                '^' => {
                    guard = Some(Transform {
                        pos: IVec2::new(x as _, y as _),
                        rot: IVec2::new(0, -1),
                    });
                }
                _ => (),
            }

            room_bound.x = room_bound.x.max(x as _);
        }
        room_bound.y = room_bound.y.max(y as _);
    }

    Scene {
        wall_positions,
        guard: guard.expect("some guard in input"),
        room_bound,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let Scene {
        wall_positions,
        mut guard,
        room_bound,
    } = parse(input);

    let mut guard_path = HashSet::new();

    while IVec2::ZERO.cmple(guard.pos).all() && guard.pos.cmple(room_bound).all() {
        guard_path.insert(guard.pos);
        let mut guard_next_pos = guard.pos + guard.rot;

        while wall_positions.contains(&guard_next_pos) {
            guard.rot = IVec2::Y.rotate(guard.rot);
            guard_next_pos = guard.pos + guard.rot;
        }

        guard.pos = guard_next_pos;
    }

    Ok(guard_path.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, process(input)?);
        Ok(())
    }
}

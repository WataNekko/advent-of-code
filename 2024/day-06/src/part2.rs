use std::collections::HashSet;

use glam::IVec2;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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
        mut wall_positions,
        mut guard,
        room_bound,
    } = parse(input);

    let guard_starting_point = guard;
    let mut possible_obstruction_placements = HashSet::new();

    loop {
        do_guard_move(&mut guard, &wall_positions);
        if !is_in_room_bound(guard.pos, room_bound) {
            break;
        };

        if guard.pos != guard_starting_point.pos {
            possible_obstruction_placements.insert(guard.pos);
        }
    }

    let count = possible_obstruction_placements
        .into_iter()
        .filter(|&obstruction| {
            wall_positions.insert(obstruction);
            let is_path_loop = is_path_loop(guard_starting_point, &wall_positions, room_bound);
            wall_positions.remove(&obstruction);

            is_path_loop
        })
        .count();

    Ok(count)
}

fn do_guard_move(guard: &mut Transform, walls: &HashSet<IVec2>) {
    let mut guard_next_pos = guard.pos + guard.rot;

    while walls.contains(&guard_next_pos) {
        guard.rot = IVec2::Y.rotate(guard.rot);
        guard_next_pos = guard.pos + guard.rot;
    }

    guard.pos = guard_next_pos;
}

fn is_in_room_bound(pos: IVec2, room_bound: IVec2) -> bool {
    IVec2::ZERO.cmple(pos).all() && pos.cmple(room_bound).all()
}

fn is_path_loop(mut guard: Transform, walls: &HashSet<IVec2>, room_bound: IVec2) -> bool {
    let mut walked_path = HashSet::new();

    loop {
        walked_path.insert(guard);

        do_guard_move(&mut guard, walls);
        if !is_in_room_bound(guard.pos, room_bound) {
            break false;
        }

        if walked_path.contains(&guard) {
            break true;
        }
    }
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
        assert_eq!(6, process(input)?);
        Ok(())
    }
}

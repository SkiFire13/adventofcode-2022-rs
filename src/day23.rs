#[allow(unused_imports)]
use super::prelude::*;
type Input = FxHashSet<(isize, isize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect()
}

fn round_simulator() -> impl FnMut(&mut FxHashSet<(isize, isize)>) -> bool {
    let mut proposals = FxHashMap::default();
    let mut tmp_positions = FxHashSet::default();
    let mut directions = VecDeque::from([
        ([0, 1, 2], (0, -1)),
        ([5, 6, 7], (0, 1)),
        ([0, 3, 5], (-1, 0)),
        ([2, 4, 7], (1, 0)),
    ]);
    move |positions| {
        let mut moved = false;
        proposals.clear();
        tmp_positions.clear();

        let move_position = |x, y| {
            let occupied = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .map(|(dx, dy)| positions.contains(&(x + dx, y + dy)));

            if occupied != [false; 8] {
                for (to_check, (dx, dy)) in &directions {
                    if to_check.iter().all(|&neighbour| !occupied[neighbour]) {
                        return Some((x + dx, y + dy));
                    }
                }
            }

            None
        };

        for &(x, y) in &*positions {
            if let Some(pos) = move_position(x, y) {
                *proposals.entry(pos).or_insert(0) += 1;
            }
        }

        for &(x, y) in &*positions {
            if let Some(pos) = move_position(x, y) {
                if proposals[&pos] == 1 {
                    tmp_positions.insert(pos);
                    moved = true;
                    continue;
                }
            }
            tmp_positions.insert((x, y));
        }

        std::mem::swap(positions, &mut tmp_positions);
        directions.rotate_left(1);
        moved
    }
}

pub fn part1(input: &Input) -> usize {
    let mut positions = input.clone();

    let mut simulator = round_simulator();
    for _ in 0..10 {
        simulator(&mut positions);
    }

    let &minx = positions.iter().map(|(x, _)| x).min().unwrap();
    let &maxx = positions.iter().map(|(x, _)| x).max().unwrap();
    let &miny = positions.iter().map(|(_, y)| y).min().unwrap();
    let &maxy = positions.iter().map(|(_, y)| y).max().unwrap();

    itertools::iproduct!(minx..=maxx, miny..=maxy)
        .filter(|pos| !positions.contains(pos))
        .count()
}

pub fn part2(input: &Input) -> usize {
    let mut positions = input.clone();

    let mut simulator = round_simulator();
    let mut round = 1;
    while simulator(&mut positions) {
        round += 1;
    }
    round
}

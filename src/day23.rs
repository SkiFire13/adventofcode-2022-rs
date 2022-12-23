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
    let mut new_positions = FxHashSet::default();
    let mut directions = VecDeque::from([
        ([0, 1, 2], (0, -1)),
        ([5, 6, 7], (0, 1)),
        ([0, 3, 5], (-1, 0)),
        ([2, 4, 7], (1, 0)),
    ]);
    move |positions| {
        let mut moved = 0;
        new_positions.clear();

        'positions: for &(x, y) in &*positions {
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
                        moved += 1;
                        if !new_positions.insert((x + dx, y + dy)) {
                            new_positions.remove(&(x + dx, y + dy));
                            new_positions.insert((x, y));
                            new_positions.insert((x + 2 * dx, y + 2 * dy));
                            moved -= 2;
                        }
                        continue 'positions;
                    }
                }
            }

            new_positions.insert((x, y));
        }

        std::mem::swap(positions, &mut new_positions);
        directions.rotate_left(1);
        moved != 0
    }
}

pub fn part1(input: &Input) -> usize {
    let mut positions = input.clone();

    let mut simulator = round_simulator();
    for _ in 0..10 {
        simulator(&mut positions);
    }

    let (mut minx, mut maxx) = (isize::MAX, isize::MIN);
    let (mut miny, mut maxy) = (isize::MAX, isize::MIN);
    for &(x, y) in &positions {
        (minx, maxx) = (min(minx, x), max(maxx, x));
        (miny, maxy) = (min(miny, y), max(maxy, y));
    }
    ((maxx - minx + 1) * (maxy - miny + 1)) as usize - positions.len()
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

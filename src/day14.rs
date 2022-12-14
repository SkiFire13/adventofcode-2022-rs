#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<(isize, isize)>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(',').expect("Invalid input");
                    let x = x.parse().expect("Invalid input");
                    let y = y.parse().expect("Invalid input");
                    (x, y)
                })
                .collect()
        })
        .collect()
}

fn prepare_filled(input: &Input) -> FxHashSet<(isize, isize)> {
    let mut filled = FxHashSet::default();
    for line in input {
        let mut curr = line[0];
        for &next in &line[1..] {
            let xs = min(curr.0, next.0)..=max(curr.0, next.0);
            let ys = min(curr.1, next.1)..=max(curr.1, next.1);
            filled.extend(itertools::iproduct!(xs, ys));
            curr = next;
        }
    }
    filled
}

fn solve<const FLOOR_PRESENT: bool>(input: &Input) -> usize {
    let mut filled = prepare_filled(input);
    let max_y = filled.iter().map(|&(_, y)| y).max().unwrap();

    let mut sand_count = 0;
    while !FLOOR_PRESENT || !filled.contains(&(500, 0)) {
        let mut sand_pos = (500, 0);
        'sand: loop {
            if !FLOOR_PRESENT && sand_pos.1 == max_y {
                return sand_count;
            }

            if FLOOR_PRESENT && sand_pos.1 == max_y + 1 {
                filled.insert(sand_pos);
                sand_count += 1;
                break;
            }

            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                let new_pos = (sand_pos.0 + dx, sand_pos.1 + dy);
                if !filled.contains(&new_pos) {
                    sand_pos = new_pos;
                    continue 'sand;
                }
            }

            filled.insert(sand_pos);
            sand_count += 1;
            break;
        }
    }
    sand_count
}

pub fn part1(input: &Input) -> usize {
    solve::<false>(input)
}

pub fn part2(input: &Input) -> usize {
    solve::<true>(input)
}

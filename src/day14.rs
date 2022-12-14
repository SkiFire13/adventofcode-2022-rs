#[allow(unused_imports)]
use super::prelude::*;
type Input = FxHashSet<(isize, isize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(',').expect("Invalid input");
                    let x = x.parse::<isize>().expect("Invalid input");
                    let y = y.parse::<isize>().expect("Invalid input");
                    (x, y)
                })
                .tuple_windows()
        })
        .flat_map(|(prev, next)| {
            let xs = min(prev.0, next.0)..=max(prev.0, next.0);
            let ys = min(prev.1, next.1)..=max(prev.1, next.1);
            itertools::iproduct!(xs, ys)
        })
        .collect()
}

fn solve<const FLOOR_PRESENT: bool>(input: &Input) -> usize {
    let mut filled = input.clone();
    let max_y = filled.iter().map(|&(_, y)| y).max().unwrap();
    let wall_count = filled.len();

    fn solve_rec<const FLOOR_PRESENT: bool>(
        (x, y): (isize, isize),
        filled: &mut FxHashSet<(isize, isize)>,
        max_y: isize,
    ) -> ControlFlow<()> {
        if !FLOOR_PRESENT && y == max_y {
            return ControlFlow::Break(());
        }

        if !(FLOOR_PRESENT && y == max_y + 1) {
            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                let next = (x + dx, y + dy);
                if !filled.contains(&next) {
                    solve_rec::<FLOOR_PRESENT>(next, filled, max_y)?;
                }
            }
        }

        filled.insert((x, y));
        ControlFlow::Continue(())
    }

    solve_rec::<FLOOR_PRESENT>((500, 0), &mut filled, max_y);
    filled.len() - wall_count
}

pub fn part1(input: &Input) -> usize {
    solve::<false>(input)
}

pub fn part2(input: &Input) -> usize {
    solve::<true>(input)
}

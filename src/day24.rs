#[allow(unused_imports)]
use super::prelude::*;
type Input = (
    (isize, isize),
    HashMap<(isize, isize), Vec<(isize, isize)>>,
    (isize, isize),
);

pub fn input_generator(input: &str) -> Input {
    let (start_line, rest) = input.split_once('\n').unwrap();
    let startx = start_line.chars().position(|c| c == '.').unwrap() as isize - 1;
    let (rest, end_line) = rest.rsplit_once('\n').unwrap();
    let endx = end_line.chars().position(|c| c == '.').unwrap() as isize - 1;
    let endy = rest.lines().count() as isize;
    let blizzards = rest
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let (x, y) = (x as isize, y as isize);
                Some(match c {
                    '.' | '#' => return None,
                    '>' => ((x - 1, y), vec![(1, 0)]),
                    '<' => ((x - 1, y), vec![(-1, 0)]),
                    'v' => ((x - 1, y), vec![(0, 1)]),
                    '^' => ((x - 1, y), vec![(0, -1)]),
                    _ => panic!(),
                })
            })
        })
        .collect();
    ((startx, -1), blizzards, (endx, endy))
}

fn solve(input: &Input, end_state: usize) -> usize {
    let (start, mut blizzards, end) = input.clone();

    let lenx = end.0 + 1;
    let leny = end.1;

    let mut iter = 0;
    let mut curr_stack = vec![(start, 0)];
    let mut next_stack = Vec::new();
    let mut seen = HashSet::new();

    loop {
        seen.clear();

        blizzards = std::mem::take(&mut blizzards)
            .into_iter()
            .flat_map(|((x, y), ds)| {
                ds.into_iter().map(move |(dx, dy)| {
                    (((x + dx + lenx) % lenx, (y + dy + leny) % leny), (dx, dy))
                })
            })
            .into_group_map();

        for (pos @ (x, y), mut state) in curr_stack.drain(..) {
            if pos == end && state == end_state {
                return iter;
            }
            if pos == end && state % 2 == 0 {
                state += 1;
            }
            if pos == start && state % 2 == 1 {
                state += 1;
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                let new_pos @ (x, y) = (x + dx, y + dy);
                if new_pos == start
                    || new_pos == end
                    || (0 <= x
                        && x < lenx
                        && 0 <= y
                        && y < leny
                        && !blizzards.contains_key(&new_pos))
                        && seen.insert((new_pos, state))
                {
                    next_stack.push((new_pos, state));
                }
            }
        }

        std::mem::swap(&mut curr_stack, &mut next_stack);
        iter += 1;
    }
}

pub fn part1(input: &Input) -> usize {
    solve(input, 0)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 2)
}

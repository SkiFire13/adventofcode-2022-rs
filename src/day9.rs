#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<((isize, isize), usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (dir, steps) = line.split_once(' ').expect("Invalid input");
            let dir = match dir {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Invalid input"),
            };
            let steps = steps.parse().expect("Invalid input");
            (dir, steps)
        })
        .collect()
}

fn update(prev: (isize, isize), next: &mut (isize, isize)) -> bool {
    match (prev.0 - next.0, prev.1 - next.1) {
        (-1..=1, -1..=1) => false,
        (dx, dy) => {
            next.0 += dx.signum();
            next.1 += dy.signum();
            true
        }
    }
}

fn rope<const N: usize>(input: &Input) -> usize {
    let mut rope = [(0, 0); N];
    let mut seen = HashSet::from([rope[N - 1]]);
    for &((dx, dy), n) in input {
        for _ in 0..n {
            rope[0].0 += dx;
            rope[0].1 += dy;
            if (0..N - 1).all(|i| update(rope[i], &mut rope[i + 1])) {
                seen.insert(rope[N - 1]);
            }
        }
    }
    seen.len()
}

pub fn part1(input: &Input) -> usize {
    rope::<2>(input)
}

pub fn part2(input: &Input) -> usize {
    rope::<10>(input)
}

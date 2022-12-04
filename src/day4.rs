#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize, usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').expect("Invalid input");
            let (a1, a2) = a.split_once('-').expect("Invalid input");
            let (b1, b2) = b.split_once('-').expect("Invalid input");
            let a1 = a1.parse().expect("Invalid input");
            let a2 = a2.parse().expect("Invalid input");
            let b1 = b1.parse().expect("Invalid input");
            let b2 = b2.parse().expect("Invalid input");
            (a1, a2, b1, b2)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(a1, a2, b1, b2)| (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2))
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(a1, a2, b1, b2)| a1 <= b2 && a2 >= b1)
        .count()
}

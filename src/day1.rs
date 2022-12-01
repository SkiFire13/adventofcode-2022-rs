#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<usize>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse().expect("Invalid input"))
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .max()
        .expect("Invalid input")
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|elf| Reverse(elf.iter().sum::<usize>()))
        .k_smallest(3)
        .map(|Reverse(sum)| sum)
        .sum()
}

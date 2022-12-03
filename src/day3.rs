#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u64, u64)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.as_bytes().split_at(line.len() / 2);
            (to_bitset(left), to_bitset(right))
        })
        .collect()
}

fn to_bitset(s: &[u8]) -> u64 {
    s.iter()
        .map(|b| match b {
            b'a'..=b'z' => b - b'a' + 1,
            b'A'..=b'Z' => b - b'A' + 27,
            _ => panic!("Invalid input"),
        })
        .fold(0, |acc, b| acc | (1 << b))
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|&(left, right)| (left & right).trailing_zeros())
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .chunks_exact(3)
        .map(|chunk| {
            let [(a1, a2), (b1, b2), (c1, c2)]: [_; 3] = chunk.try_into().unwrap();
            ((a1 | a2) & (b1 | b2) & (c1 | c2)).trailing_zeros()
        })
        .sum()
}

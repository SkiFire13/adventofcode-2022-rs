#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| ((line[0] - b'A') as usize, (line[2] - b'X') as usize))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .copied()
        .map(|(a, x)| {
            let move_score = x + 1;
            let game_result = (3 + x - a + 1) % 3;
            move_score + 3 * game_result
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .copied()
        .map(|(a, x)| {
            let move_to_play = (a + x + 3 - 1) % 3;
            let move_score = move_to_play + 1;
            let game_result = x;
            game_result * 3 + move_score
        })
        .sum()
}

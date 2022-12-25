#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<&'a [u8]>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &Input) -> String {
    let mut n = input
        .iter()
        .map(|&line| {
            let mut acc = 0i64;
            for &b in line {
                acc = acc * 5
                    + match b {
                        b'0' => 0,
                        b'1' => 1,
                        b'2' => 2,
                        b'-' => -1,
                        b'=' => -2,
                        _ => panic!("Invalid input"),
                    }
            }
            acc as u64
        })
        .sum::<u64>();
    let mut out = Vec::new();
    while n != 0 {
        let (carry, character) = match n % 5 {
            0 => (0, '0'),
            1 => (0, '1'),
            2 => (0, '2'),
            3 => (1, '='),
            4 => (1, '-'),
            _ => unreachable!(),
        };
        n = n / 5 + carry;
        out.push(character);
    }
    out.into_iter().rev().collect()
}

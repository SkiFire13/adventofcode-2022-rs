#[allow(unused_imports)]
use super::prelude::*;
type Input<'input> = &'input [u8];

pub fn input_generator(input: &str) -> Input<'_> {
    input.as_bytes()
}

struct ByteCounter {
    counts: [usize; 256],
    unique: usize,
}

impl ByteCounter {
    fn add(&mut self, b: u8) {
        self.counts[b as usize] += 1;
        self.unique += (self.counts[b as usize] == 1) as usize;
    }
    fn remove(&mut self, b: u8) {
        self.counts[b as usize] -= 1;
        self.unique -= (self.counts[b as usize] == 0) as usize;
    }
}

fn position_n_distinct(input: &[u8], n: usize) -> usize {
    let mut counter = ByteCounter {
        counts: [0; 256],
        unique: 0,
    };
    input[0..n].iter().for_each(|&b| counter.add(b));
    if counter.unique == n {
        return n;
    }

    n + 1
        + input
            .windows(n + 1)
            .position(|c| {
                let &[old, .., new] = c else { panic!("Invalid input") };
                counter.remove(old);
                counter.add(new);
                counter.unique == n
            })
            .expect("Invalid input")
}

pub fn part1(input: &Input) -> usize {
    position_n_distinct(input, 4)
}

pub fn part2(input: &Input) -> usize {
    position_n_distinct(input, 14)
}

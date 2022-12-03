#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(Vec<u8>, Vec<u8>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.as_bytes().split_at(line.len() / 2);
            let mut left = left.to_vec();
            left.sort_unstable();
            let mut right = right.to_vec();
            right.sort_unstable();
            (left, right)
        })
        .collect()
}

fn intersection<T, I, J>(iter1: I, iter2: J) -> impl Iterator<Item = T>
where
    T: Ord,
    I: IntoIterator<Item = T>,
    J: IntoIterator<Item = T>,
{
    itertools::merge_join_by(iter1, iter2, T::cmp)
        .filter_map(|merged| merged.both())
        .map(|(x, _)| x)
        .dedup()
}

fn byte_to_value(b: u8) -> usize {
    match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => panic!(),
    }
    .into()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(left, right)| {
            let (&both,) = intersection(left, right)
                .collect_tuple()
                .expect("Invalid input");
            byte_to_value(both)
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .chunks_exact(3)
        .map(|chunk| {
            let ((a1, a2), (b1, b2), (c1, c2)) = (&chunk[0], &chunk[1], &chunk[2]);
            let a = itertools::merge(a1, a2);
            let b = itertools::merge(b1, b2);
            let c = itertools::merge(c1, c2);
            let (&all,) = intersection(a, intersection(b, c))
                .collect_tuple()
                .expect("Invalid input");
            byte_to_value(all)
        })
        .sum()
}

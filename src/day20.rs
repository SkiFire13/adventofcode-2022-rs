#[allow(unused_imports)]
use super::prelude::*;
type Input = VecDeque<i64>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(len: usize, get: impl Fn(usize) -> i64, iter: impl Iterator<Item = usize>) -> i64 {
    let mut list = (0..len).collect::<VecDeque<_>>();

    for n in iter {
        let pos = list.iter().position(|&i| i == n).unwrap();
        list.rotate_left(pos);

        let offset = get(n);
        list.pop_front();
        if offset >= 0 {
            list.insert((offset as usize) % list.len(), n);
        } else {
            list.insert(list.len() - (-offset) as usize % list.len(), n);
        }
    }

    let zero = list.iter().position(|&n| get(n) == 0).unwrap();
    get(list[(zero + 1000) % list.len()])
        + get(list[(zero + 2000) % list.len()])
        + get(list[(zero + 3000) % list.len()])
}

pub fn part1(input: &Input) -> i64 {
    solve(input.len(), |n| input[n], 0..input.len())
}

pub fn part2(input: &Input) -> i64 {
    solve(
        input.len(),
        |n| input[n] * 811589153,
        (0..10).flat_map(|_| 0..input.len()),
    )
}

#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Vec<u8>>, Vec<(usize, usize, usize)>);

pub fn input_generator(input: &str) -> Input {
    let (start, moves) = input.split_once("\n\n").expect("Invalid input");

    let (start, idxs) = start.rsplit_once('\n').expect("Invalid input");
    let mut start_state = vec![Vec::new(); idxs.split("   ").count()];
    for line in start.lines().rev() {
        for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
            if chunk[1] != b' ' {
                start_state[i].push(chunk[1]);
            }
        }
    }

    let moves = moves
        .lines()
        .map(|line| {
            let (n, rest) = line[5..].split_once(" from ").expect("Invalid input");
            let (from, to) = rest.split_once(" to ").expect("Invalid input");
            let n = n.parse().expect("Invalid input");
            let from = from.parse().expect("Invalid input");
            let to = to.parse().expect("Invalid input");
            (n, from, to)
        })
        .collect();

    (start_state, moves)
}

pub fn part1(input: &Input) -> String {
    let (start, moves) = input;
    let mut state = start.clone();

    for &(n, from, to) in moves {
        let (from, to) = state.get2_mut(from - 1, to - 1);
        to.extend(from.drain(from.len() - n..).rev());
    }

    state
        .into_iter()
        .map(|state| *state.last().expect("Invalid input") as char)
        .collect()
}

pub fn part2(input: &Input) -> String {
    let (start, moves) = input;
    let mut state = start.clone();

    for &(n, from, to) in moves {
        let (from, to) = state.get2_mut(from - 1, to - 1);
        to.extend(from.drain(from.len() - n..));
    }

    state
        .into_iter()
        .map(|state| *state.last().expect("Invalid input") as char)
        .collect()
}

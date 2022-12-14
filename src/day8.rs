#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

pub fn part1(input: &Input) -> usize {
    let mut seen = input.to_set(|_, _, _| false);

    fn mark(input: &Input, seen: &mut GridSet, iter: impl Iterator<Item = (usize, usize)>) {
        let mut max = None;
        for p in iter {
            if Some(input[p]) > max {
                seen.insert(p);
                max = Some(input[p]);
                if max == Some(9) {
                    return;
                }
            }
        }
    }

    for x in 0..input.w() {
        mark(input, &mut seen, (0..input.h()).map(|y| (x, y)));
        mark(input, &mut seen, (0..input.h()).rev().map(|y| (x, y)));
    }
    for y in 0..input.h() {
        mark(input, &mut seen, (0..input.w()).map(|x| (x, y)));
        mark(input, &mut seen, (0..input.w()).rev().map(|x| (x, y)));
    }

    seen.count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter_by_row()
        .map(|(x, y)| {
            fn seen(input: &Input, iter: impl Clone + Iterator<Item = (usize, usize)>) -> usize {
                let mut iter = iter;
                let center = input[iter.next().unwrap()];
                iter.take_while_inclusive(|&p| input[p] < center).count()
            }

            seen(input, (x..input.w()).map(|x| (x, y)))
                * seen(input, (0..=x).rev().map(|x| (x, y)))
                * seen(input, (y..input.h()).map(|y| (x, y)))
                * seen(input, (0..=y).rev().map(|y| (x, y)))
        })
        .max()
        .expect("Invalid input")
}

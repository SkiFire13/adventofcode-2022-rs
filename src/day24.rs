#[allow(unused_imports)]
use super::prelude::*;
type Input = ((usize, usize), [Vec<u64>; 4]);

pub fn input_generator(input: &str) -> Input {
    let lenx = input.find('\n').expect("Invalid input") - 2;
    let leny = input.lines().count() - 2;
    assert!(leny < 64);
    let mut blizzards = array::from_fn(|_| vec![(1 << leny) - 1; lenx]);

    for (y, line) in input.lines().dropping(1).dropping_back(1).enumerate() {
        for (x, c) in line.chars().dropping(1).dropping_back(1).enumerate() {
            match c {
                '.' => {}
                '^' => blizzards[0][x] &= !(1 << y),
                'v' => blizzards[1][x] &= !(1 << y),
                '>' => blizzards[2][x] &= !(1 << y),
                '<' => blizzards[3][x] &= !(1 << y),
                _ => panic!(),
            }
        }
    }

    ((lenx, leny), blizzards)
}

fn solve((lenx, leny): (usize, usize), mut blizzards: [Vec<u64>; 4], max_iter: usize) -> usize {
    let mut steps = 0;
    let mut iter = 0;
    let mut reachable = vec![0; lenx];

    let mask_inside = (1u64 << leny) - 1;
    let mask_last = 1 << (leny - 1);

    loop {
        let shift_up = |m| ((m >> 1) | ((m & 1) << (leny - 1)));
        let shift_down = |m| ((m << 1) | ((m & mask_last) >> (leny - 1)));
        blizzards[0].iter_mut().for_each(|m| *m = shift_up(*m));
        blizzards[1].iter_mut().for_each(|m| *m = shift_down(*m));
        blizzards[2].rotate_right(1);
        blizzards[3].rotate_left(1);
        steps += 1;

        if (reachable[lenx - 1] & mask_last != 0 && iter % 2 == 0)
            || (reachable[0] & 1 != 0 && iter % 2 == 1)
        {
            reachable.fill(0);
            iter += 1;
            if iter == max_iter {
                return steps;
            }
            continue;
        }

        let mut prev = if iter % 2 == 0 { 1 } else { 0 };
        let last = if iter % 2 == 1 { mask_last } else { 0 };
        for x in 0..lenx {
            let prev = std::mem::replace(&mut prev, reachable[x]);
            let next = reachable.get(x + 1).copied().unwrap_or(last);
            reachable[x] |= (reachable[x] >> 1) | (reachable[x] << 1) | prev | next;
            reachable[x] &= blizzards[0][x] & blizzards[1][x] & blizzards[2][x] & blizzards[3][x];
            reachable[x] &= mask_inside;
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let ((lenx, leny), blizzards) = input.clone();
    solve((lenx, leny), blizzards, 1)
}

pub fn part2(input: &Input) -> usize {
    let ((lenx, leny), blizzards) = input.clone();
    solve((lenx, leny), blizzards, 3)
}

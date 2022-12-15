#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<((isize, isize), (isize, isize))>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once("x=").expect("Invalid input");
            let (sx, rest) = rest.split_once(", y=").expect("Invalid input");
            let (sy, rest) = rest.split_once(":").expect("Invalid input");
            let (_, rest) = rest.split_once("x=").expect("Invalid input");
            let (bx, by) = rest.split_once(", y=").expect("Invalid input");
            let sx = sx.parse().expect("Invalid input");
            let sy = sy.parse().expect("Invalid input");
            let bx = bx.parse().expect("Invalid input");
            let by = by.parse().expect("Invalid input");
            ((sx, sy), (bx, by))
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    const ROW_TARGET: isize = 2000000;

    let mut invalid_ranges = Vec::new();
    let mut beacons = Vec::new();
    for &((sx, sy), (bx, by)) in input {
        let db = isize::abs_diff(sx, bx) + isize::abs_diff(sy, by);
        let drow = isize::abs_diff(sy, ROW_TARGET);
        if let Some(slack) = db.checked_sub(drow) {
            let slack = slack as isize;
            invalid_ranges.push((sx - slack, sx + slack + 1));
        }
        if by == ROW_TARGET {
            beacons.push(bx);
        }
    }

    invalid_ranges.sort_unstable_by_key(|&(s, e)| (s, Reverse(e)));
    beacons.sort_unstable();
    beacons.dedup();

    let mut beacons = &beacons[..];
    let mut last_end = isize::MIN;
    let mut count = 0;

    for (start, end) in invalid_ranges {
        if last_end >= end {
            continue;
        }
        let start = max(last_end, start);
        last_end = end;

        eat_while(&mut beacons, |&b| b < start);
        let removed = eat_while(&mut beacons, |&b| b < end);

        count += (end - start) as usize - removed.len();
    }

    count
}

pub fn part2(input: &Input) -> usize {
    for row_target in 0..=4000000 {
        let mut invalid_ranges = Vec::new();
        for &((sx, sy), (bx, by)) in input {
            let db = isize::abs_diff(sx, bx) + isize::abs_diff(sy, by);
            let drow = isize::abs_diff(sy, row_target);
            if let Some(slack) = db.checked_sub(drow) {
                let slack = slack as isize;
                invalid_ranges.push((sx - slack, sx + slack + 1));
            }
        }

        invalid_ranges.sort_unstable_by_key(|&(s, e)| (s, Reverse(e)));

        let mut last_end = 0;

        for (start, end) in invalid_ranges {
            if last_end < start {
                return (last_end * 4000000 + row_target) as usize;
            }
            last_end = max(end, last_end);
        }
    }

    panic!("Invalid input")
}

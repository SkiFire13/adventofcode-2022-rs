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

    let mut last_end = isize::MIN;

    let reachable = input
        .iter()
        .filter_map(|&((sx, sy), (bx, by))| {
            let db = isize::abs_diff(sx, bx) + isize::abs_diff(sy, by);
            let drow = isize::abs_diff(sy, ROW_TARGET);
            let slack = db.checked_sub(drow)? as isize;
            Some((sx - slack, sx + slack + 1))
        })
        .sorted_unstable_by_key(|&(s, e)| (s, Reverse(e)))
        .map(|(start, end)| {
            let start = max(last_end, start);
            let end = max(last_end, end);
            last_end = end;
            (end - start) as usize
        })
        .sum::<usize>();

    let beacons = input
        .iter()
        .filter(|&&(_, (_, by))| by == ROW_TARGET)
        .map(|(_, (bx, _))| bx)
        .unique()
        .count();

    reachable - beacons
}

pub fn part2(input: &Input) -> usize {
    (0..=4000000)
        .into_par_iter()
        .find_map_any(|row_target| {
            let mut last_end = 0;
            input
                .iter()
                .filter_map(|&((sx, sy), (bx, by))| {
                    let db = isize::abs_diff(sx, bx) + isize::abs_diff(sy, by);
                    let drow = isize::abs_diff(sy, row_target);
                    let slack = db.checked_sub(drow)? as isize;
                    Some((sx - slack, sx + slack + 1))
                })
                .sorted_unstable_by_key(|&(s, e)| (s, Reverse(e)))
                .find_map(|(start, end)| {
                    let v = (last_end < start).then(|| (last_end * 4000000 + row_target) as usize);
                    last_end = max(end, last_end);
                    v
                })
        })
        .expect("Invalid input")
}

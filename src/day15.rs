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
    let mut stack = vec![((0, 0), (4000000, 4000000))];

    'stack: while let Some(((minx, miny), (w, h))) = stack.pop() {
        if w == 0 || h == 0 {
            continue 'stack;
        }

        for &((sx, sy), (bx, by)) in input {
            let db = isize::abs_diff(sx, bx) + isize::abs_diff(sy, by);
            if [(0, 0), (0, h - 1), (w - 1, 0), (w - 1, h - 1)]
                .into_iter()
                .map(|(dx, dy)| (minx + dx, miny + dy))
                .map(|(x, y)| isize::abs_diff(sx, x) + isize::abs_diff(sy, y))
                .all(|dc| dc <= db)
            {
                continue 'stack;
            }
        }

        if w == 1 && h == 1 {
            return (minx * 4000000 + miny) as usize;
        }

        let (w1, w2, h1, h2) = (w / 2, (w + 1) / 2, h / 2, (h + 1) / 2);

        stack.extend([
            ((minx, miny), (w1, h1)),
            ((minx + w1, miny), (w2, h1)),
            ((minx, miny + h1), (w1, h2)),
            ((minx + w1, miny + h1), (w2, h2)),
        ]);
    }

    panic!("Invalid input")
}

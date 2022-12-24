#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<(i8, i8)>;

pub fn input_generator(input: &str) -> Input {
    let lenx = input.find('\n').expect("Invalid input") as isize - 2;
    let leny = input.lines().count() as isize - 2;

    let mut blizzards = Grid::with_dimensions(lenx as usize, leny as usize);
    for (y, line) in input.lines().dropping(1).dropping_back(1).enumerate() {
        for (x, c) in line.chars().dropping(1).dropping_back(1).enumerate() {
            let (x, y) = (x as isize, y as isize);
            match c {
                '.' => {}
                '>' => blizzards[(x, y)] = (1, 0),
                '<' => blizzards[(x, y)] = (-1, 0),
                'v' => blizzards[(x, y)] = (0, 1),
                '^' => blizzards[(x, y)] = (0, -1),
                _ => panic!(),
            }
        }
    }

    blizzards
}

fn solve(blizzards: &Input, max_iter: usize) -> usize {
    let lenx = blizzards.w() as isize;
    let leny = blizzards.h() as isize;
    let (mut start, mut end) = ((0, -1), (lenx - 1, leny));

    let mut iter = 0;
    let mut queue = BinaryHeap::from([(Reverse(0), Reverse(0), start)]);
    let mut seen = FxHashSet::default();

    while let Some((_, Reverse(steps), pos @ (x, y))) = queue.pop() {
        if pos == end {
            if iter == max_iter {
                return steps as usize;
            }
            (start, end) = (end, start);
            queue.clear();
            seen.clear();
            iter += 1;
        }

        'next: for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
            let (x, y) = (x + dx, y + dy);
            if (x, y) != start && (x, y) != end {
                if !(0 <= x && x < lenx && 0 <= y && y < leny) {
                    continue 'next;
                }
                for (bdx, bdy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let bx = (x - bdx * (steps + 1)).rem_euclid(lenx) as usize;
                    let by = (y - bdy * (steps + 1)).rem_euclid(leny) as usize;
                    if blizzards[(bx, by)] == (bdx as i8, bdy as i8) {
                        continue 'next;
                    }
                }
                if !seen.insert((x, y, steps)) {
                    continue 'next;
                }
            }
            let (endx, endy) = end;
            let lower_bound = steps + 1 + isize::abs(endx - x) + isize::abs(endy - y);
            queue.push((Reverse(lower_bound), Reverse(steps + 1), (x, y)));
        }
    }

    panic!("Invalid input")
}

pub fn part1(input: &Input) -> usize {
    solve(input, 0)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 2)
}

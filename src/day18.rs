#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line.split(',').collect_tuple().expect("Invalid input");
            let x = x.parse().expect("Invalid input");
            let y = y.parse().expect("Invalid input");
            let z = z.parse().expect("Invalid input");
            (x, y, z)
        })
        .collect()
}

fn solve(input: &Input, count_sides: impl Fn(usize, usize, usize) -> usize) -> usize {
    input.iter().map(|&(x, y, z)| count_sides(x, y, z)).sum()
}

fn lens(input: &Input) -> (usize, usize, usize) {
    input
        .iter()
        .fold((0, 0, 0), |(lenx, leny, lenz), &(x, y, z)| {
            (max(lenx, x + 1), max(leny, y + 1), max(lenz, z + 1))
        })
}

pub fn part1(input: &Input) -> usize {
    let (lenx, leny, lenz) = lens(input);

    let mut set = Grid3D::with_dimensions(lenx + 2, leny + 2, lenz + 2).into_set();
    for &(x, y, z) in input {
        set.insert((x + 1, y + 1, z + 1));
    }

    solve(input, |x, y, z| {
        set.plus_neighbours((x + 1, y + 1, z + 1))
            .filter(|&p| !set.contains(p))
            .count()
    })
}

pub fn part2(input: &Input) -> usize {
    let (lenx, leny, lenz) = lens(input);

    let mut set = Grid3D::with_dimensions(lenx + 2, leny + 2, lenz + 2).into_set();
    for &(x, y, z) in input {
        set.insert((x + 1, y + 1, z + 1));
    }

    let mut wrapper = Grid3D::with_dimensions(lenx + 2, leny + 2, lenz + 2).into_set();
    let mut stack = vec![(0, 0, 0)];
    while let Some((x, y, z)) = stack.pop() {
        for (x, y, z) in set.plus_neighbours((x, y, z)) {
            if !set.contains((x, y, z)) && wrapper.insert((x, y, z)) {
                stack.push((x, y, z))
            }
        }
    }

    solve(input, |x, y, z| {
        wrapper
            .plus_neighbours((x + 1, y + 1, z + 1))
            .filter(|&p| wrapper.contains(p))
            .count()
    })
}

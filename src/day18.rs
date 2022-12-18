#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(i16, i16, i16)>;

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

fn solve(input: &Input, check_block: impl Fn(i16, i16, i16) -> bool) -> usize {
    input
        .iter()
        .flat_map(|&(x, y, z)| {
            [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
        })
        .filter(|&(x, y, z)| check_block(x, y, z))
        .count()
}

fn lens(input: &Input) -> (usize, usize, usize) {
    let (mut lenx, mut leny, mut lenz) = (0, 0, 0);
    for &(x, y, z) in input {
        lenx = max(lenx, x as usize + 1);
        leny = max(leny, y as usize + 1);
        lenz = max(lenz, z as usize + 1);
    }
    (lenx, leny, lenz)
}

pub fn part1(input: &Input) -> usize {
    let (lenx, leny, lenz) = lens(input);

    let mut set = vec![false; lenx * leny * lenz];
    for &(x, y, z) in input {
        let (x, y, z) = (x as usize, y as usize, z as usize);
        set[x + lenx * (y + leny * z)] = true;
    }
    solve(input, |x, y, z| {
        let xok = 0 <= x && x < lenx as i16;
        let yok = 0 <= y && y < leny as i16;
        let zok = 0 <= z && z < lenz as i16;
        let (x, y, z) = (x as usize, y as usize, z as usize);
        !(xok && yok && zok && set[x + lenx * (y + leny * z)])
    })
}

pub fn part2(input: &Input) -> usize {
    let (lenx, leny, lenz) = lens(input);
    let init = |_, _| (i16::MAX, 0);
    let mut minmaxx = Grid::with_dimensions_init(leny + 1, lenz + 1, init);
    let mut minmaxy = Grid::with_dimensions_init(lenx + 1, lenz + 1, init);
    let mut minmaxz = Grid::with_dimensions_init(lenx + 1, leny + 1, init);

    for &(x, y, z) in input {
        let merge = |minmax: &mut Grid<_>, (p0, p1), c| {
            let (minc, maxc) = &mut minmax[(p0 as usize, p1 as usize)];
            *minc = min(*minc, c);
            *maxc = max(*maxc, c);
        };
        merge(&mut minmaxx, (y, z), x);
        merge(&mut minmaxy, (x, z), y);
        merge(&mut minmaxz, (x, y), z);
    }
    let check_inside = |minmax: &Grid<_>, (p0, p1), c| {
        let Some(&(min, max)) = minmax.iget((p0 as isize, p1 as isize)) else { return false };
        min <= c && c <= max
    };
    solve(input, |x, y, z| {
        !check_inside(&minmaxx, (y, z), x)
            || !check_inside(&minmaxy, (x, z), y)
            || !check_inside(&minmaxz, (x, y), z)
    })
}

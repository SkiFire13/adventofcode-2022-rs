#[allow(unused_imports)]
use super::prelude::*;
type Input = (GridSet, isize);

pub fn input_generator(input: &str) -> Input {
    let (mut minx, mut maxx, mut maxy) = (usize::MAX, 0, 0);

    let segments = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(',').expect("Invalid input");
                    let x = x.parse::<isize>().expect("Invalid input");
                    let y = y.parse::<isize>().expect("Invalid input");
                    (x, y)
                })
                .tuple_windows()
        })
        .inspect(|&((ax, ay), (bx, by))| {
            minx = min(minx, min(ax, bx) as usize);
            maxx = max(maxx, max(ax, bx) as usize);
            maxy = max(maxy, max(ay, by) as usize);
        })
        .collect::<Vec<_>>();

    let offset = minx as isize - 1;
    let mut grid = Grid::with_dimensions(maxx - minx + 3, maxy + 2).into_set();
    segments
        .into_iter()
        .flat_map(|(prev, next)| {
            let xs = min(prev.0, next.0)..=max(prev.0, next.0);
            let ys = min(prev.1, next.1)..=max(prev.1, next.1);
            itertools::iproduct!(xs, ys)
        })
        .for_each(|(x, y)| grid[(x - offset, y)] = true);

    (grid, offset)
}

fn solve<const FLOOR_PRESENT: bool>(input: &Input) -> usize {
    fn solve_rec<const FLOOR_PRESENT: bool>(
        (x, y): (isize, isize),
        output: &mut (GridSet, usize, isize, isize),
        bounds: (isize, isize),
    ) -> ControlFlow<()> {
        let (maxx, maxy) = bounds;

        if !FLOOR_PRESENT && y == maxy - 2 {
            return ControlFlow::Break(());
        }

        if !(FLOOR_PRESENT && y == maxy - 1) {
            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                let (filled, _, ly, ry) = output;
                if FLOOR_PRESENT && x == 0 && dx == -1 {
                    *ly = max(*ly, maxy - y - 1);
                } else if FLOOR_PRESENT && x == maxx - 1 && dx == 1 {
                    *ry = max(*ry, maxy - y - 1);
                } else if !filled.contains(((x + dx) as usize, (y + dy) as usize)) {
                    solve_rec::<FLOOR_PRESENT>((x + dx, y + dy), output, bounds)?;
                }
            }
        }

        let (filled, count, _, _) = output;
        filled.insert((x as usize, y as usize));
        *count += 1;

        ControlFlow::Continue(())
    }

    let (filled, dx) = input.clone();
    let maxx = filled.w() as isize;
    let maxy = filled.h() as isize;
    let mut output = (filled, 0, 0, 0);
    solve_rec::<FLOOR_PRESENT>((500 - dx, 0), &mut output, (maxx, maxy));

    let (_, count, ly, ry) = output;
    count + (ly * (ly + 1) / 2 + ry * (ry + 1) / 2) as usize
}

pub fn part1(input: &Input) -> usize {
    solve::<false>(input)
}

pub fn part2(input: &Input) -> usize {
    solve::<true>(input)
}

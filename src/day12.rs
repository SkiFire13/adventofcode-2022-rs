#[allow(unused_imports)]
use super::prelude::*;
type Input = ((usize, usize), (usize, usize), Grid<u8>);

pub fn input_generator(input: &str) -> Input {
    let mut start = None;
    let mut end = None;
    let grid = Grid::from_input_chars(input, |c, x, y| match c {
        'S' => {
            start = Some((x, y));
            0
        }
        'E' => {
            end = Some((x, y));
            25
        }
        'a'..='z' => c as u8 - b'a',
        _ => panic!("Invalid input"),
    });
    let start = start.expect("Invalid input");
    let end = end.expect("Invalid input");
    (start, end, grid)
}

fn find_shortest_path<I>(grid: &Grid<u8>, end: (usize, usize), initial: I) -> usize
where
    I: Iterator<Item = (usize, usize)>,
{
    let mk_item = |steps, pos| {
        let min_steps = steps - grid[pos] as usize;
        (Reverse(min_steps), Reverse(steps), pos)
    };
    let initial = initial.map(|pos| mk_item(0, pos));
    let mut queue = BinaryHeap::from_iter(initial);
    let mut seen = grid.map_ref(|_, _, _| false);

    while let Some((_, Reverse(steps), pos)) = queue.pop() {
        if !seen[pos] {
            seen[pos] = true;
            if pos == end {
                return steps;
            }

            for new_pos in grid.plus_neighbours(pos) {
                if grid[new_pos] <= grid[pos] + 1 && !seen[new_pos] {
                    queue.push(mk_item(steps + 1, new_pos));
                }
            }
        }
    }

    panic!("Invalid input")
}

pub fn part1(input: &Input) -> usize {
    let &(start, end, ref grid) = input;
    find_shortest_path(grid, end, [start].into_iter())
}

pub fn part2(input: &Input) -> usize {
    let &(start, end, ref grid) = input;
    let a_points = (0..grid.w())
        .flat_map(|x| (0..grid.h()).map(move |y| (x, y)))
        .filter(|&pos| grid[pos] == 0);
    find_shortest_path(grid, end, a_points.chain([start]))
}

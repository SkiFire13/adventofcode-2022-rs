#[allow(unused_imports)]
use super::prelude::*;
type Input = ((usize, usize), (usize, usize), Grid<u8>);

pub fn input_generator(input: &str) -> Input {
    let mut start = None;
    let mut end = None;
    let grid = Grid::from_input_chars(input, |c, x, y| {
        start = if c == 'S' { Some((x, y)) } else { start };
        end = if c == 'E' { Some((x, y)) } else { end };
        match c {
            'S' => b'a',
            'E' => b'z',
            'a'..='z' => c as u8,
            _ => panic!("Invalid input"),
        }
    });
    let start = start.expect("Invalid input");
    let end = end.expect("Invalid input");
    (start, end, grid)
}

fn find_shortest_path<I>(grid: &Grid<u8>, end: (usize, usize), initial: I) -> usize
where
    I: Iterator<Item = (usize, usize)>,
{
    let initial = initial.map(|pos| (Reverse(0), pos));
    let mut queue = BinaryHeap::from_iter(initial);
    let mut seen = grid.to_set(|_, _, _| false);

    while let Some((Reverse(steps), pos)) = queue.pop() {
        if seen.insert(pos) {
            if pos == end {
                return steps;
            }

            for new_pos in grid.plus_neighbours(pos) {
                if grid[new_pos] <= grid[pos] + 1 && !seen.contains(new_pos) {
                    queue.push((Reverse(steps + 1), new_pos));
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
    let &(_, end, ref grid) = input;
    let a_points = grid
        .iter()
        .filter_map(|(pos, &cell)| (cell == b'a').then(|| pos));
    find_shortest_path(grid, end, a_points)
}

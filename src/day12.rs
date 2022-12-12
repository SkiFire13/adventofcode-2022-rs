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

fn find_shortest_path<F>(grid: &Grid<u8>, is_target: F, initial: (usize, usize)) -> usize
where
    F: Fn((usize, usize)) -> bool,
{
    let mut queue = VecDeque::from([(0, initial)]);
    let mut seen = grid.to_set(|_, _, _| false);

    while let Some((steps, pos)) = queue.pop_front() {
        if is_target(pos) {
            return steps;
        }

        for new_pos in grid.plus_neighbours(pos) {
            if grid[pos] <= grid[new_pos] + 1 && seen.insert(new_pos) {
                queue.push_back((steps + 1, new_pos));
            }
        }
    }

    panic!("Invalid input")
}

pub fn part1(input: &Input) -> usize {
    let &(start, end, ref grid) = input;
    find_shortest_path(grid, |pos| pos == start, end)
}

pub fn part2(input: &Input) -> usize {
    let &(_, end, ref grid) = input;
    find_shortest_path(grid, |pos| grid[pos] == b'a', end)
}

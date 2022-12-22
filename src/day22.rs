#[allow(unused_imports)]
use super::prelude::*;
type Input = (Grid<Tile>, Vec<Instruction>);

#[derive(PartialEq, Eq)]
pub enum Tile {
    Void,
    Empty,
    Wall,
}

pub enum Instruction {
    Step(isize),
    Turn(isize),
}

const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (-1, 0);
const EAST: (isize, isize) = (1, 0);

const TURN_LEFT: isize = 1;
const TURN_RIGHT: isize = -1;

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

pub fn input_generator(input: &str) -> Input {
    let (map, raw_instructions) = input.split_once("\n\n").expect("Invalid input");

    let map = map.lines().map(str::as_bytes).collect::<Vec<_>>();
    let map_width = map.iter().map(|bs| bs.len()).max().expect("Invalid input");
    let map_height = map.len();

    let map = Grid::with_dimensions_init(map_width, map_height, |x, y| {
        match map[y].get(x).unwrap_or(&b' ') {
            b' ' => Tile::Void,
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            _ => panic!("Invalid input"),
        }
    });

    let mut instructions = Vec::new();
    for instr in raw_instructions.split_inclusive(&['L', 'R']) {
        let steps = instr
            .strip_suffix(&['L', 'R'])
            .unwrap_or(instr)
            .parse()
            .expect("Invalid input");
        instructions.push(Instruction::Step(steps));
        match instr.chars().next_back().expect("Invalid input") {
            'L' => instructions.push(Instruction::Turn(TURN_LEFT)),
            'R' => instructions.push(Instruction::Turn(TURN_RIGHT)),
            _ => {}
        }
    }

    (map, instructions)
}

const SQUARE_SIDE: usize = 50;

fn identify_squares(grid: &Grid<Tile>) -> ([(usize, usize); 6], Grid<usize>) {
    let width = grid.w() / SQUARE_SIDE;
    let height = grid.h() / SQUARE_SIDE;

    let square_positions_iter = itertools::iproduct!(0..height, 0..width)
        .map(|(y, x)| (x, y))
        .filter(|&(x, y)| grid[(x * SQUARE_SIDE, y * SQUARE_SIDE)] != Tile::Void);
    let squares_positions = <[(usize, usize); 6]>::from_iter(square_positions_iter);
    let squares_grid = Grid::with_dimensions_init(width, height, |x, y| {
        (0..6)
            .find(|&i| squares_positions[i] == (x, y))
            .unwrap_or(6)
    });
    (squares_positions, squares_grid)
}

fn simulate_with_edges(
    input: &Input,
    squares_positions: [(usize, usize); 6],
    edges: [[(usize, usize); 4]; 6],
) -> usize {
    let (grid, instructions) = input;

    let map_pos = |(x, y, side)| {
        let (x, y) = (x as usize, y as usize);
        let (side_x, side_y) = squares_positions[side];
        (side_x * SQUARE_SIDE + x, side_y * SQUARE_SIDE + y)
    };

    let (mut pos, mut dir) = ((0, 0, 0), EAST);
    for instr in instructions {
        match instr {
            &Instruction::Turn(d) => dir = (d * dir.1, -d * dir.0),
            &Instruction::Step(n) => {
                for _ in 0..n {
                    let max_side = SQUARE_SIDE as isize - 1;
                    let (x, y, square) = pos;
                    let (next_x, next_y, (next_square, next_rot)) = match dir {
                        EAST if x == max_side => (0, y, edges[square][RIGHT]),
                        WEST if x == 0 => (max_side, y, edges[square][LEFT]),
                        SOUTH if y == max_side => (x, 0, edges[square][BOTTOM]),
                        NORTH if y == 0 => (x, max_side, edges[square][TOP]),
                        _ => (x + dir.0, y + dir.1, (square, 0)),
                    };
                    let (next_x, next_y, next_dir) = match next_rot {
                        0 => (next_x, next_y, dir),
                        1 => (next_y, max_side - next_x, (dir.1, -dir.0)),
                        2 => (max_side - next_x, max_side - next_y, (-dir.0, -dir.1)),
                        3 => (max_side - next_y, next_x, (-dir.1, dir.0)),
                        _ => unreachable!(),
                    };
                    let next_pos = (next_x, next_y, next_square);
                    if grid[map_pos(next_pos)] == Tile::Wall {
                        break;
                    }
                    (pos, dir) = (next_pos, next_dir);
                }
            }
        }
    }

    let (x, y) = map_pos(pos);
    let dir_value = match dir {
        EAST => 0,
        SOUTH => 1,
        WEST => 2,
        NORTH => 3,
        _ => unreachable!(),
    };
    1000 * (y + 1) + 4 * (x + 1) + dir_value
}

pub fn part1(input: &Input) -> usize {
    let (grid, _) = input;
    let (squares_positions, squares_grid) = identify_squares(&grid);

    fn set_edges_chain(
        edges: &mut [[(usize, usize); 4]; 6],
        iter: impl Iterator<Item = usize> + Clone,
        (before, after): (usize, usize),
    ) {
        iter.filter(|&i| i < 6)
            .cycle()
            .tuple_windows()
            .take_while_inclusive(|&(i, j)| i < j)
            .for_each(|(i, j)| {
                edges[i][after].0 = j;
                edges[j][before].0 = i;
            });
    }

    let mut edges = [[(7, 0); 4]; 6];
    for y in 0..squares_grid.h() {
        let iter = (0..squares_grid.w()).map(|x| squares_grid[(x, y)]);
        set_edges_chain(&mut edges, iter, (3, 1));
    }
    for x in 0..squares_grid.w() {
        let iter = (0..squares_grid.h()).map(|y| squares_grid[(x, y)]);
        set_edges_chain(&mut edges, iter, (0, 2));
    }

    simulate_with_edges(input, squares_positions, edges)
}

pub fn part2(input: &Input) -> usize {
    let (grid, _) = input;
    let (squares_positions, squares_grid) = identify_squares(&grid);

    let mut edges = [[(7, 0); 4]; 6];
    for ((x, y), &i) in squares_grid.iter().filter(|&(_, &i)| i < 6) {
        if x != 0 && squares_grid[(x - 1, y)] < 6 {
            edges[i][LEFT] = (squares_grid[(x - 1, y)], 0);
            edges[squares_grid[(x - 1, y)]][RIGHT] = (i, 0);
        }
        if y != 0 && squares_grid[(x, y - 1)] < 6 {
            edges[i][TOP] = (squares_grid[(x, y - 1)], 0);
            edges[squares_grid[(x, y - 1)]][BOTTOM] = (i, 0);
        }
    }
    for _ in 0..6 {
        for i in 0..6 {
            for d in 0..4 {
                let Some(&(n1 @ 0..=5, rot1)) = edges[i].get(d) else { continue };
                let Some(&(n2 @ 0..=5, rot2)) = edges[n1].get((d + 1 + 4 - rot1) % 4) else { continue };
                let rot = (1 + rot1 + rot2) % 4;
                edges[i][(d + 1) % 4] = (n2, rot);
                edges[n2][(3 - rot + d) % 4] = (i, (4 - rot) % 4);
            }
        }
    }

    simulate_with_edges(input, squares_positions, edges)
}

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

fn identify_squares(grid: &Grid<Tile>) -> ([(usize, usize); 6], Grid<usize>, usize) {
    let mut square_side = num::integer::gcd(grid.w(), grid.h());
    if square_side == grid.w() || square_side == grid.h() {
        square_side /= 2;
    }

    let width = grid.w() / square_side;
    let height = grid.h() / square_side;

    let square_positions_iter = itertools::iproduct!(0..height, 0..width)
        .map(|(y, x)| (x, y))
        .filter(|&(x, y)| grid[(x * square_side, y * square_side)] != Tile::Void);
    let squares_positions = <[(usize, usize); 6]>::from_iter(square_positions_iter);
    let squares_grid = Grid::with_dimensions_init(width, height, |x, y| {
        (0..6)
            .find(|&i| squares_positions[i] == (x, y))
            .unwrap_or(6)
    });
    (squares_positions, squares_grid, square_side)
}

fn simulate_with_edges(
    input: &Input,
    squares_positions: [(usize, usize); 6],
    edges: [[(usize, usize, isize); 4]; 6],
    square_side: usize,
) -> usize {
    let (grid, instructions) = input;

    let map_pos = |(x, y, side)| {
        let (x, y) = (x as usize, y as usize);
        let (side_x, side_y) = squares_positions[side];
        (side_x * square_side + x, side_y * square_side + y)
    };

    let (mut pos, mut dir, mut flip) = ((0, 0, 0), EAST, 1);
    for instr in instructions {
        match instr {
            &Instruction::Turn(d) => {
                {
                    let (real_x, real_y) = map_pos(pos);
                    for y in 0..grid.h() {
                        for x in 0..grid.w() {
                            match grid[(x, y)] {
                                Tile::Wall => {
                                    assert_ne!((x, y), (real_x, real_y));
                                    print!("#")
                                }
                                Tile::Void => {
                                    assert_ne!((x, y), (real_x, real_y));
                                    print!(" ")
                                }
                                Tile::Empty => {
                                    print!(
                                        "{}",
                                        if (x, y) == (real_x, real_y) && flip == 1 {
                                            match dir {
                                                (1, 0) => ">",
                                                (-1, 0) => "<",
                                                (0, 1) => "v",
                                                (0, -1) => "^",
                                                _ => unreachable!(),
                                            }
                                        } else {
                                            "."
                                        }
                                    )
                                }
                            }
                        }
                        print!("{}", " ".repeat(10));
                        let y = grid.h() - 1 - y;
                        for x in 0..grid.w() {
                            match grid[(x, y)] {
                                Tile::Wall => {
                                    assert_ne!((x, y), (real_x, real_y));
                                    print!("#")
                                }
                                Tile::Void => {
                                    assert_ne!((x, y), (real_x, real_y));
                                    print!(" ")
                                }
                                Tile::Empty => {
                                    print!(
                                        "{}",
                                        if (x, y) == (real_x, real_y) && flip == -1 {
                                            match dir {
                                                (1, 0) => ">",
                                                (-1, 0) => "<",
                                                (0, 1) => "^",
                                                (0, -1) => "v",
                                                _ => unreachable!(),
                                            }
                                        } else {
                                            "."
                                        }
                                    )
                                }
                            }
                        }
                        print!("{}", " ".repeat(10));
                        println!()
                    }
                    println!();
                }

                println!(
                    "Turning {}",
                    match d {
                        TURN_LEFT => "left",
                        TURN_RIGHT => "right",
                        _ => unreachable!(),
                    }
                );
                dir = (flip * d * dir.1, flip * -d * dir.0)
            }
            &Instruction::Step(n) => {
                println!("Stepping {n}");
                for _ in 0..n {
                    {
                        let (real_x, real_y) = map_pos(pos);
                        for y in 0..grid.h() {
                            for x in 0..grid.w() {
                                match grid[(x, y)] {
                                    Tile::Wall => {
                                        assert_ne!((x, y), (real_x, real_y));
                                        print!("#")
                                    }
                                    Tile::Void => {
                                        assert_ne!((x, y), (real_x, real_y));
                                        print!(" ")
                                    }
                                    Tile::Empty => {
                                        print!(
                                            "{}",
                                            if (x, y) == (real_x, real_y) && flip == 1 {
                                                match dir {
                                                    (1, 0) => ">",
                                                    (-1, 0) => "<",
                                                    (0, 1) => "v",
                                                    (0, -1) => "^",
                                                    _ => unreachable!(),
                                                }
                                            } else {
                                                "."
                                            }
                                        )
                                    }
                                }
                            }
                            print!("{}", " ".repeat(10));
                            let y = grid.h() - 1 - y;
                            for x in 0..grid.w() {
                                match grid[(x, y)] {
                                    Tile::Wall => {
                                        assert_ne!((x, y), (real_x, real_y));
                                        print!("#")
                                    }
                                    Tile::Void => {
                                        assert_ne!((x, y), (real_x, real_y));
                                        print!(" ")
                                    }
                                    Tile::Empty => {
                                        print!(
                                            "{}",
                                            if (x, y) == (real_x, real_y) && flip == -1 {
                                                match dir {
                                                    (1, 0) => ">",
                                                    (-1, 0) => "<",
                                                    (0, 1) => "^",
                                                    (0, -1) => "v",
                                                    _ => unreachable!(),
                                                }
                                            } else {
                                                "."
                                            }
                                        )
                                    }
                                }
                            }
                            print!("{}", " ".repeat(10));
                            println!()
                        }
                        println!();
                    }

                    let max_side = square_side as isize - 1;
                    let (x, y, square) = pos;
                    let (next_x, next_y, (next_square, next_rot, flip_change)) = match dir {
                        EAST if x == max_side => (0, y, edges[square][RIGHT]),
                        WEST if x == 0 => (max_side, y, edges[square][LEFT]),
                        SOUTH if y == max_side => (x, 0, edges[square][BOTTOM]),
                        NORTH if y == 0 => (x, max_side, edges[square][TOP]),
                        _ => (x + dir.0, y + dir.1, (square, 0, 1)),
                    };
                    let (next_x, next_y, next_dir) = match (next_rot, flip_change) {
                        (0, -1) => (x, y, (-dir.0, -dir.1)),
                        (0, 1) => (next_x, next_y, dir),
                        (1, 1) => (next_y, max_side - next_x, (dir.1, -dir.0)),
                        (2, 1) => (max_side - next_x, max_side - next_y, (-dir.0, -dir.1)),
                        (3, 1) => (max_side - next_y, next_x, (-dir.1, dir.0)),
                        _ => unreachable!(),
                    };
                    let next_pos = (next_x, next_y, next_square);
                    let next_flip = flip * flip_change;
                    if grid[map_pos(next_pos)] == Tile::Wall {
                        break;
                    }
                    (pos, dir, flip) = (next_pos, next_dir, next_flip);
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
    return 0;
    let (grid, _) = input;
    let (squares_positions, squares_grid, square_side) = identify_squares(&grid);

    fn set_edges_chain(
        edges: &mut [[(usize, usize, isize); 4]; 6],
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

    let mut edges = [[(7, 0, 1); 4]; 6];
    for y in 0..squares_grid.h() {
        let iter = (0..squares_grid.w()).map(|x| squares_grid[(x, y)]);
        set_edges_chain(&mut edges, iter, (3, 1));
    }
    for x in 0..squares_grid.w() {
        let iter = (0..squares_grid.h()).map(|y| squares_grid[(x, y)]);
        set_edges_chain(&mut edges, iter, (0, 2));
    }

    simulate_with_edges(input, squares_positions, edges, square_side)
}

pub fn part2(input: &Input) -> usize {
    println!("Part 3: {}", part3(input));
    return 0;

    let (grid, _) = input;
    let (squares_positions, squares_grid, square_side) = identify_squares(&grid);

    let mut edges = [[(7, 0, 1); 4]; 6];
    for ((x, y), &i) in squares_grid.iter().filter(|&(_, &i)| i < 6) {
        if x != 0 && squares_grid[(x - 1, y)] < 6 {
            edges[i][LEFT] = (squares_grid[(x - 1, y)], 0, 1);
            edges[squares_grid[(x - 1, y)]][RIGHT] = (i, 0, 1);
        }
        if y != 0 && squares_grid[(x, y - 1)] < 6 {
            edges[i][TOP] = (squares_grid[(x, y - 1)], 0, 1);
            edges[squares_grid[(x, y - 1)]][BOTTOM] = (i, 0, 1);
        }
    }
    for _ in 0..6 {
        for i in 0..6 {
            for d in 0..4 {
                let Some(&(n1 @ 0..=5, rot1, _)) = edges[i].get(d) else { continue };
                let Some(&(n2 @ 0..=5, rot2, _)) = edges[n1].get((d + 1 + 4 - rot1) % 4) else { continue };
                let rot = (1 + rot1 + rot2) % 4;
                edges[i][(d + 1) % 4] = (n2, rot, 1);
                edges[n2][(3 - rot + d) % 4] = (i, (4 - rot) % 4, 1);
            }
        }
    }

    simulate_with_edges(input, squares_positions, edges, square_side)
}

pub fn part3(input: &Input) -> usize {
    let (grid, _) = input;
    let (squares_positions, squares_grid, square_side) = identify_squares(&grid);

    let lid = (0..6)
        .min_by_key(|&i| {
            let (square_x, square_y) = squares_positions[i];
            (square_y, Reverse(square_x))
        })
        .unwrap();
    let (lid_x, lid_y) = squares_positions[lid];
    let left_lid = (lid_x != 0).then(|| (lid_x - 1, lid_y));
    let bottom_lid = (lid_y != squares_grid.h() - 1).then(|| (lid_x, lid_y + 1));
    let lid_connected = [left_lid, bottom_lid]
        .into_iter()
        .flatten()
        .map(|pos| squares_grid[pos])
        .filter(|&square| square != 6)
        .exactly_one()
        .expect("Invalid input");

    let mut edges = [[(7, 0, 1); 4]; 6];
    for ((x, y), &i) in squares_grid.iter().filter(|&(_, &i)| i < 6) {
        if x != 0 && squares_grid[(x - 1, y)] < 6 {
            edges[i][LEFT] = (squares_grid[(x - 1, y)], 0, 1);
            edges[squares_grid[(x - 1, y)]][RIGHT] = (i, 0, 1);
        }
        if y != 0 && squares_grid[(x, y - 1)] < 6 {
            edges[i][TOP] = (squares_grid[(x, y - 1)], 0, 1);
            edges[squares_grid[(x, y - 1)]][BOTTOM] = (i, 0, 1);
        }
    }
    for _ in 0..6 {
        for i in 0..6 {
            for d in 0..4 {
                let Some(&(n1 @ 0..=5, rot1, _)) = edges[i].get(d) else { continue };
                let Some(&(n2 @ 0..=5, rot2, _)) = edges[n1].get((d + 1 + 4 - rot1) % 4) else { continue };
                let rot = (1 + rot1 + rot2) % 4;
                edges[i][(d + 1) % 4] = (n2, rot, 1);
                edges[n2][(3 - rot + d) % 4] = (i, (4 - rot) % 4, 1);
            }
        }
    }

    for square in 0..6 {
        if square != lid && square != lid_connected {
            for edge @ &mut (conn, _, _) in &mut edges[square] {
                if conn == lid {
                    *edge = (square, 0, -1);
                }
            }
        }
    }
    for edge @ &mut (conn, _, _) in &mut edges[lid] {
        if conn != lid_connected {
            *edge = (lid, 0, -1);
        }
    }

    simulate_with_edges(input, squares_positions, edges, square_side)
}

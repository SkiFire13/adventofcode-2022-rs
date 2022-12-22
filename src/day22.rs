#[allow(unused_imports)]
use super::prelude::*;
type Input = (Grid<Tile>, Vec<Movement>);

#[derive(PartialEq, Eq)]
pub enum Tile {
    Void,
    Empty,
    Wall,
}

pub enum Movement {
    Move(isize),
    Turn(isize),
}

const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (-1, 0);
const EAST: (isize, isize) = (1, 0);

const LEFT: isize = 1;
const RIGHT: isize = -1;

pub fn input_generator(input: &str) -> Input {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let map = map.lines().map(str::as_bytes).collect::<Vec<_>>();
    let map_width = map.iter().map(|bs| bs.len()).max().unwrap();
    let map_height = map.len();

    let map = Grid::with_dimensions_init(map_width, map_height, |x, y| {
        match map[y].get(x).unwrap_or(&b' ') {
            b' ' => Tile::Void,
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            _ => panic!(),
        }
    });

    let mut movements = Vec::new();

    let mut directions = directions.as_bytes();
    loop {
        let movement = eat_while(&mut directions, |n| n.is_ascii_digit());
        movements.push(Movement::Move(
            movement
                .iter()
                .fold(0, |acc, d| acc * 10 + (d - b'0') as isize),
        ));

        if directions.is_empty() {
            break;
        }

        match eat_copy(&mut directions) {
            b'L' => movements.push(Movement::Turn(LEFT)),
            b'R' => movements.push(Movement::Turn(RIGHT)),
            _ => panic!(),
        }
    }

    (map, movements)
}

pub fn part1(input: &Input) -> isize {
    let (grid, directions) = input;
    let xbounds = (0..grid.w())
        .map(|x| {
            (0..grid.h())
                .filter(|&y| grid[(x, y)] != Tile::Void)
                .minmax()
                .into_option()
                .map(|(miny, maxy)| (miny as isize, maxy as isize))
                .unwrap()
        })
        .collect::<Vec<_>>();
    let ybounds = (0..grid.h())
        .map(|y| {
            (0..grid.w())
                .filter(|&x| grid[(x, y)] != Tile::Void)
                .minmax()
                .into_option()
                .map(|(minx, maxx)| (minx as isize, maxx as isize))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut dir = EAST;
    let mut pos = (ybounds[0].0, 0);
    while grid[pos] != Tile::Empty {
        pos.0 += 1;
    }

    for movement in directions {
        match movement {
            &Movement::Turn(d) => {
                dir = (d * dir.1, -d * dir.0);
            }
            &Movement::Move(n) => {
                for _ in 0..n {
                    let next = match dir {
                        EAST if ybounds[pos.1 as usize].1 == pos.0 => {
                            (ybounds[pos.1 as usize].0, pos.1)
                        }
                        WEST if ybounds[pos.1 as usize].0 == pos.0 => {
                            (ybounds[pos.1 as usize].1, pos.1)
                        }
                        SOUTH if xbounds[pos.0 as usize].1 == pos.1 => {
                            (pos.0, xbounds[pos.0 as usize].0)
                        }
                        NORTH if xbounds[pos.0 as usize].0 == pos.1 => {
                            (pos.0, xbounds[pos.0 as usize].1)
                        }
                        _ => (pos.0 + dir.0, pos.1 + dir.1),
                    };

                    if grid[next] == Tile::Wall {
                        break;
                    }

                    pos = next;
                }
            }
        }
    }

    1000 * (pos.1 + 1)
        + 4 * (pos.0 + 1)
        + match dir {
            EAST => 0,
            SOUTH => 1,
            WEST => 2,
            NORTH => 3,
            _ => panic!(),
        }
}

pub fn part2(input: &Input) -> isize {
    let (grid, directions) = input;
    let xbounds = (0..grid.w())
        .map(|x| {
            (0..grid.h())
                .filter(|&y| grid[(x, y)] != Tile::Void)
                .minmax()
                .into_option()
                .map(|(miny, maxy)| (miny as isize, maxy as isize))
                .unwrap()
        })
        .collect::<Vec<_>>();
    let ybounds = (0..grid.h())
        .map(|y| {
            (0..grid.w())
                .filter(|&x| grid[(x, y)] != Tile::Void)
                .minmax()
                .into_option()
                .map(|(minx, maxx)| (minx as isize, maxx as isize))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut dir = EAST;
    let mut pos = (ybounds[0].0, 0);
    while grid[pos] != Tile::Empty {
        pos.0 += 1;
    }

    let lx1 = ybounds[0].0;
    let lx2 = 0;
    let rx1 = grid.w() as isize - 1;
    let rx2 = ybounds[xbounds[ybounds[0].0 as usize].1 as usize].1;
    let rx3 = ybounds[grid.h() - 1].1;
    let ty1 = xbounds[0].0;
    let ty2 = 0;
    let by1 = grid.h() as isize - 1;
    let by2 = xbounds[ybounds[0].0 as usize].1;
    let by3 = xbounds[grid.w() - 1].1;

    for movement in directions {
        match movement {
            &Movement::Turn(d) => {
                dir = (d * dir.1, -d * dir.0);
            }
            &Movement::Move(n) => {
                for _ in 0..n {
                    let (x, y) = pos;
                    let (next_pos, next_dir) = match dir {
                        EAST if ybounds[y as usize].1 == x => {
                            if x == rx1 {
                                ((rx2, by2 - y), WEST)
                            } else if x == rx3 {
                                ((rx2 - (grid.h() as isize - 1 - y), by2), NORTH)
                            } else if y >= ty1 {
                                ((rx1, by2 - y), WEST)
                            } else {
                                ((y - by3 + rx2, by3), NORTH)
                            }
                        }
                        WEST if ybounds[y as usize].0 == x => {
                            if y <= by3 {
                                ((lx2, by2 - y), EAST)
                            } else if x == lx1 {
                                ((y - by3 - 1, ty1), SOUTH)
                            } else if y <= by2 {
                                ((lx1, by2 - y), EAST)
                            } else {
                                ((lx1 + y - (by2 + 1), ty2), SOUTH)
                            }
                        }
                        SOUTH if xbounds[x as usize].1 == y => {
                            if y == by1 {
                                ((x + rx2 + 1, ty2), SOUTH)
                            } else if y == by2 {
                                ((rx3, x - rx2 + by1), WEST)
                            } else {
                                ((rx2, x - rx1 + ty1 - 1), WEST)
                            }
                        }
                        NORTH if xbounds[x as usize].0 == y => {
                            if x < lx1 {
                                ((lx1, by3 + 1 + x), EAST)
                            } else if x <= rx2 {
                                ((lx2, x - lx1 + by2 + 1), EAST)
                            } else {
                                ((x - (rx2 + 1), by1), NORTH)
                            }
                        }
                        _ => ((x + dir.0, y + dir.1), dir),
                    };

                    if grid[next_pos] == Tile::Wall {
                        break;
                    }

                    pos = next_pos;
                    dir = next_dir;
                }
            }
        }
    }

    1000 * (pos.1 + 1)
        + 4 * (pos.0 + 1)
        + match dir {
            EAST => 0,
            SOUTH => 1,
            WEST => 2,
            NORTH => 3,
            _ => panic!(),
        }
}

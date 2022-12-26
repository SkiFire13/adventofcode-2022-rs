#[allow(unused_imports)]
use super::prelude::*;
type Input = GridSet;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| match c {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid input"),
    })
    .into_set()
}

struct Simulator {
    iter: usize,
    width: usize,
    elfs: Vec<(usize, usize)>,
    grid: Vec<bool>,
    candidates: Vec<u8>,
}

impl Simulator {
    fn new(input: &Input) -> Self {
        let (width, height) = (input.w() + 1 + 80, input.h() + 1 + 80);
        let mut grid = vec![false; width * height];
        let candidates = vec![0; width * height];

        let elfs = input
            .iter_set()
            .map(|(x, y)| x + 20 + width * (y + 20))
            .inspect(|&pos| grid[pos] = true)
            .map(|pos| (pos, 0))
            .collect();

        Self { iter: 0, width, elfs, grid, candidates }
    }

    fn step(&mut self) -> bool {
        #[rustfmt::skip]
        let neighbours = [ (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1) ];
        let directions = [
            ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
            ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
            ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
            ([(1, -1), (1, 0), (1, 1)], (1, 0)),
        ];

        'elfs: for (pos, candidate) in &mut self.elfs {
            let pos = *pos as isize;

            let occupied = |(dx, dy)| self.grid[(pos + dx + dy * self.width as isize) as usize];

            if neighbours.iter().any(|&(dx, dy)| occupied((dx, dy))) {
                for i in 0..4 {
                    let (to_check, (dx, dy)) = directions[(i + self.iter) % 4];
                    if to_check.iter().all(|&neighbour| !occupied(neighbour)) {
                        let candidate_pos = (pos + dx + dy * self.width as isize) as usize;
                        *candidate = candidate_pos;
                        self.candidates[candidate_pos] += 1;
                        continue 'elfs;
                    }
                }
            }
        }

        let (mut minx, mut maxx) = (usize::MAX, 0);
        let (mut miny, mut maxy) = (usize::MAX, 0);

        let mut moved = 0;
        for (pos, candidate_pos) in &mut self.elfs {
            let candidate_pos = std::mem::take(candidate_pos);
            if candidate_pos != 0 {
                if std::mem::take(&mut self.candidates[candidate_pos]) == 1 {
                    self.grid[*pos] = false;
                    self.grid[candidate_pos] = true;
                    *pos = candidate_pos;
                    moved += 1;

                    let (x, y) = (*pos % self.width, *pos / self.width);
                    (minx, maxx) = (min(minx, x), max(maxx, x));
                    (miny, maxy) = (min(miny, y), max(maxy, y));
                }
            }
        }

        let height = self.grid.len() / self.width;
        if minx == 0 || maxx == self.width - 1 || miny == 0 || maxy == height - 1 {
            let new_len = (self.width + 40) * (height + 40);
            self.grid.clear();
            self.grid.resize(new_len, false);
            self.candidates.clear();
            self.candidates.resize(new_len, 0);
            self.width += 40;
            for (pos, _) in &mut self.elfs {
                *pos += 20 + 20 * self.width;
                self.grid[*pos] = true;
            }
        }

        self.iter += 1;

        moved != 0
    }
}

pub fn part1(input: &Input) -> usize {
    let mut simulator = Simulator::new(input);
    for _ in 0..10 {
        simulator.step();
    }

    let (mut minx, mut maxx) = (usize::MAX, usize::MIN);
    let (mut miny, mut maxy) = (usize::MAX, usize::MIN);
    for &(pos, _) in &simulator.elfs {
        let (x, y) = (pos % simulator.width, pos / simulator.width);
        (minx, maxx) = (min(minx, x), max(maxx, x));
        (miny, maxy) = (min(miny, y), max(maxy, y));
    }
    (maxx - minx + 1) * (maxy - miny + 1) - simulator.elfs.len()
}

pub fn part2(input: &Input) -> usize {
    let mut simulator = Simulator::new(input);
    while simulator.step() {}
    simulator.iter
}

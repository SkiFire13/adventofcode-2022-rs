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

fn round_simulator(input: &Input) -> impl FnMut(&mut Input) -> bool {
    let mut directions = VecDeque::from([
        ([0, 1, 2], (0, -1)),
        ([5, 6, 7], (0, 1)),
        ([0, 3, 5], (-1, 0)),
        ([2, 4, 7], (1, 0)),
    ]);

    let (mut minx, mut maxx) = (isize::MAX, isize::MIN);
    let (mut miny, mut maxy) = (isize::MAX, isize::MIN);
    for (x, y) in input.iter_set() {
        let (x, y) = (x as isize, y as isize);
        (minx, maxx) = (min(minx, x), max(maxx, x));
        (miny, maxy) = (min(miny, y), max(maxy, y));
    }

    let mut new_set = Grid::new().into_set();

    move |set| {
        let mut moved = 0;
        new_set.vec.clear();
        new_set
            .vec
            .resize(((maxx - minx + 3) * (maxy - miny + 3)) as usize, false);
        new_set.width = (maxx - minx + 3) as usize;

        let encode = move |(x, y)| ((x + 1 - minx) as usize, (y + 1 - miny) as usize);

        (minx, maxx) = (isize::MAX, isize::MIN);
        (miny, maxy) = (isize::MAX, isize::MIN);

        'positions: for (x, y) in set.iter_set() {
            let (x, y) = (x as isize, y as isize);
            #[rustfmt::skip]
            let occupied = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1) ]
                .map(|(dx, dy)| *set.iget((x + dx, y + dy)).unwrap_or(&false));

            if occupied != [false; 8] {
                for (to_check, (dx, dy)) in &directions {
                    if to_check.iter().all(|&neighbour| !occupied[neighbour]) {
                        moved += 1;
                        if !new_set.insert(encode((x + dx, y + dy))) {
                            new_set.remove(encode((x + dx, y + dy)));
                            new_set.insert(encode((x, y)));
                            new_set.insert(encode((x + 2 * dx, y + 2 * dy)));
                            moved -= 2;
                        }
                        let (ex, ey) = encode((x + dx, y + dy));
                        let (ex, ey) = (ex as isize, ey as isize);
                        (minx, maxx) = (min(minx, ex), max(maxx, ex));
                        (miny, maxy) = (min(miny, ey), max(maxy, ey));
                        continue 'positions;
                    }
                }
            }

            let (ex, ey) = encode((x, y));
            let (ex, ey) = (ex as isize, ey as isize);
            (minx, maxx) = (min(minx, ex), max(maxx, ex));
            (miny, maxy) = (min(miny, ey), max(maxy, ey));
            new_set.insert(encode((x, y)));
        }

        std::mem::swap(set, &mut new_set);
        directions.rotate_left(1);
        moved != 0
    }
}

pub fn part1(input: &Input) -> usize {
    let mut positions = input.clone();

    let mut simulator = round_simulator(input);
    for _ in 0..10 {
        simulator(&mut positions);
    }

    let (mut minx, mut maxx) = (usize::MAX, usize::MIN);
    let (mut miny, mut maxy) = (usize::MAX, usize::MIN);
    for (x, y) in positions.iter_set() {
        (minx, maxx) = (min(minx, x), max(maxx, x));
        (miny, maxy) = (min(miny, y), max(maxy, y));
    }
    (maxx - minx + 1) * (maxy - miny + 1) - positions.count()
}

pub fn part2(input: &Input) -> usize {
    let mut positions = input.clone();

    let mut simulator = round_simulator(input);
    let mut round = 1;
    while simulator(&mut positions) {
        round += 1;
    }

    round
}

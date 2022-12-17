#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<isize>;

pub fn input_generator(input: &str) -> Input {
    input
        .bytes()
        .map(|b| match b {
            b'>' => 1,
            b'<' => -1,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn solve<const CACHE: bool>(input: &Input, n: usize) -> usize {
    let mut rocks = Vec::from([0u8; 7]);
    let mut maxy = 0;

    let mut prev_min_y = 0;
    let mut curr_relations = Vec::new();
    let mut cache = HashMap::new();
    let mut maxy_offset = 0;

    let mut i = 0;
    let mut input_idx = 0;
    while i < n {
        rocks.resize(maxy + 7, 0);

        let mut rock = [
            [(3, 0), (4, 0), (5, 0), (6, 0), (6, 0)],
            [(4, 0), (3, 1), (4, 1), (5, 1), (4, 2)],
            [(3, 0), (4, 0), (5, 0), (5, 1), (5, 2)],
            [(3, 0), (3, 1), (3, 2), (3, 3), (3, 3)],
            [(3, 0), (4, 0), (3, 1), (4, 1), (4, 1)],
        ][i % 5];
        rock.iter_mut().for_each(|(_, y)| *y += maxy + 3);

        let input_iteration_before = input_idx / input.len();

        loop {
            let dx = input[input_idx % input.len()];
            input_idx += 1;

            if rock
                .iter()
                .map(|&(x, y)| (x + dx, y))
                .all(|(x, y)| x != 0 && x != 8 && rocks[y] & (1 << x) == 0)
            {
                rock.iter_mut().for_each(|(x, _)| *x += dx);
            }

            if rock
                .iter()
                .any(|&(x, y)| y == 0 || rocks[y - 1] & (1 << x) != 0)
            {
                rock.iter().for_each(|&(x, y)| rocks[y] |= 1 << x);
                maxy = max(maxy, 1 + rock.iter().map(|&(_, y)| y).max().unwrap());
                break;
            }

            rock.iter_mut().for_each(|(_, y)| *y -= 1);
        }

        if CACHE {
            let curr_min_y = rock.iter().map(|&(_, y)| y).min().unwrap() as isize;
            let curr_min_x = rock.iter().map(|&(x, _)| x).min().unwrap();
            curr_relations.push((curr_min_x, curr_min_y - prev_min_y));
            prev_min_y = curr_min_y;

            let input_iteration_after = input_idx / input.len();
            if input_iteration_after > input_iteration_before {
                let key = (i % 5, input_idx % input.len(), take(&mut curr_relations));
                if let Some(&(cached_i, cached_maxy)) = cache.get(&key) {
                    let steps = i - cached_i;
                    let cycles = (n - i - 1) / steps;
                    i += cycles * steps;
                    maxy_offset += cycles * (maxy - cached_maxy);
                }
                cache.insert(key, (i, maxy));
            }
        }

        i += 1;
    }

    maxy + maxy_offset
}

pub fn part1(input: &Input) -> usize {
    solve::<false>(input, 2022)
}

pub fn part2(input: &Input) -> usize {
    solve::<true>(input, 1000000000000)
}

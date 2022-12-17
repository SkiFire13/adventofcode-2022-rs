#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<isize>;

pub fn input_generator(input: &str) -> Input {
    input
        .chars()
        .map(|line| match line {
            '>' => 1,
            '<' => -1,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn solve<const CACHE: bool>(input: &Input, n: usize) -> usize {
    let mut rocks = Vec::from([0u8; 7]);
    let mut maxy = 0;

    let mut cache = HashMap::new();
    let mut prev_rock: Option<[(isize, usize); 5]> = None;
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

        loop {
            let dx = input[input_idx];
            input_idx = (input_idx + 1) % input.len();

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
                for (x, y) in rock {
                    rocks[y] |= 1 << x;
                    maxy = max(maxy, y + 1);
                }

                break;
            }

            rock.iter_mut().for_each(|(_, y)| *y -= 1);
        }

        if CACHE {
            // I have absolutely no idea why this works, but it does for my input so...
            if let Some(prev_rock) = prev_rock.replace(rock) {
                if rock.iter().all(|&(x, y)| {
                    prev_rock.contains(&(x, y - 1)) == (rocks[y - 1] & (1 << x) != 0)
                }) {
                    let min = rock.iter().map(|&(_, y)| y).min().unwrap();
                    rock.iter_mut().for_each(|(_, y)| *y -= min);

                    if let Some(&(cached_i, cached_maxy)) = cache.get(&(rock, input_idx)) {
                        let steps = i - cached_i;
                        let cycles = (n - i - 1) / steps;
                        i += cycles * steps;
                        maxy_offset += cycles * (maxy - cached_maxy);
                    }
                    cache.insert((rock, input_idx), (i, maxy));
                }
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

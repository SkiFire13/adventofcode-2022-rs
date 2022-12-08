#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

pub fn part1(input: &Input) -> usize {
    let mut g = input.map_ref(|_, _, _| 0);
    for x in 0..g.width {
        let mut min = None;
        let mut y = 0;
        while min != Some(9) && y < g.height() {
            if Some(input[(x, y)]) > min {
                g[(x, y)] = 1;
                min = Some(input[(x, y)]);
            }
            y += 1;
        }

        let mut min = None;
        let mut y = g.height();
        while min != Some(9) && y > 0 {
            y -= 1;
            if Some(input[(x, y)]) > min {
                g[(x, y)] = 1;
                min = Some(input[(x, y)]);
            }
        }
    }
    for y in 0..g.height() {
        let mut min = None;
        let mut x = 0;
        while min != Some(9) && x < g.width {
            if Some(input[(x, y)]) > min {
                g[(x, y)] = 1;
                min = Some(input[(x, y)]);
            }
            x += 1;
        }

        let mut min = None;
        let mut x = g.width;
        while min != Some(9) && x > 0 {
            x -= 1;
            if Some(input[(x, y)]) > min {
                g[(x, y)] = 1;
                min = Some(input[(x, y)]);
            }
        }
    }
    g.vec.iter().sum()
}

pub fn part2(input: &Input) -> usize {
    (0..input.width)
        .flat_map(|x| (0..input.height()).map(move |y| (x, y)))
        .map(|(x, y)| {
            let mut tot = 1;

            let mut seen = 0;
            let mut prev = None;
            let mut yh = y + 1;
            while prev < Some(input[(x, y)]) && yh < input.height() {
                seen += 1;
                prev = Some(input[(x, yh)]);
                yh += 1;
            }
            tot *= seen;

            let mut seen = 0;
            let mut prev = None;
            let mut xh = x + 1;
            while prev < Some(input[(x, y)]) && xh < input.width {
                seen += 1;
                prev = Some(input[(xh, y)]);
                xh += 1;
            }
            tot *= seen;

            let mut seen = 0;
            let mut prev = None;
            let mut yh = y;
            while prev < Some(input[(x, y)]) && yh > 0 {
                yh -= 1;
                seen += 1;
                prev = Some(input[(x, yh)]);
            }
            tot *= seen;

            let mut seen = 0;
            let mut prev = None;
            let mut xh = x;
            while prev < Some(input[(x, y)]) && xh > 0 {
                xh -= 1;
                seen += 1;
                prev = Some(input[(xh, y)]);
            }
            tot *= seen;

            tot
        })
        .max()
        .unwrap()
}

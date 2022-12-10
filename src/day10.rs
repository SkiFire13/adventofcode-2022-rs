#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instr>;

#[derive(Copy, Clone)]
pub enum Instr {
    Addx(i64),
    Noop,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match &line[..4] {
            "addx" => Instr::Addx(line[5..].parse().expect("Invalid input")),
            "noop" => Instr::Noop,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn simulate(input: &Input, mut f: impl FnMut(usize, i64)) {
    let mut instr = 0;
    let mut addx_started = false;
    let mut x = 1;

    for cycle in 1..=240 {
        f(cycle, x);
        match (input[instr], std::mem::take(&mut addx_started)) {
            (Instr::Noop, _) => {}
            (Instr::Addx(_), false) => addx_started = true,
            (Instr::Addx(n), true) => x += n,
        }
        instr += if addx_started { 0 } else { 1 };
    }
}

pub fn part1(input: &Input) -> i64 {
    let mut acc = 0;
    simulate(input, |cycle, x| match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => acc += cycle as i64 * x,
        _ => {}
    });
    acc
}

pub fn part2(input: &Input) -> String {
    let mut screen = [b'.'; 240];

    simulate(input, |cycle, x| {
        if let -1..=1 = (cycle as i64 - 1) % 40 - x {
            screen[cycle - 1] = b'#';
        }
    });

    ocr_bytes_5x6(&screen)
}

#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Vec<u64>>, Vec<Monkey>);

pub struct Monkey {
    exp: u32,
    prod: u64,
    sum: u64,
    test: u64,
    true_idx: usize,
    false_idx: usize,
}

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|lines| {
            let (_, items, op, test, true_mon, false_mon) =
                lines.lines().collect_tuple().expect("Invalid input");
            let (_, items) = items.split_once(": ").expect("Invalid input");
            let items = items
                .split(", ")
                .map(|n| n.parse().expect("Invalid input"))
                .collect();
            let (op, operand) = op.rsplit_once(" ").expect("Invalid input");
            let (_, op) = op.rsplit_once(" ").expect("Invalid input");
            let (exp, prod, sum) = match (op, operand) {
                ("*", "old") => (2, 1, 0),
                ("+", operand) => (1, 1, operand.parse().expect("Invalid input")),
                ("*", operand) => (1, operand.parse().expect("Invalid input"), 0),
                _ => panic!("Invalid input"),
            };
            let (_, test) = test.rsplit_once(' ').expect("Invalid input");
            let test = test.parse().expect("Invalid input");
            let (_, true_mon) = true_mon.rsplit_once(' ').expect("Invalid input");
            let true_idx = true_mon.parse().expect("Invalid input");
            let (_, false_mon) = false_mon.rsplit_once(' ').expect("Invalid input");
            let false_idx = false_mon.parse().expect("Invalid input");
            (items, Monkey { exp, prod, sum, test, true_idx, false_idx })
        })
        .unzip()
}

fn simulate_monkeys(input: &Input, rounds: usize, reduce: impl Fn(u64) -> u64) -> usize {
    let (items, monkeys) = input;
    let mut items = items.clone();
    let mut inspected = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            while let Some(worry) = items[i].pop() {
                let new_worry = reduce(worry.pow(monkey.exp) * monkey.prod + monkey.sum);
                match new_worry % monkey.test {
                    0 => items[monkey.true_idx].push(new_worry),
                    _ => items[monkey.false_idx].push(new_worry),
                }
                inspected[i] += 1;
            }
        }
    }

    inspected.sort_unstable();
    inspected[inspected.len() - 2] * inspected[inspected.len() - 1]
}

pub fn part1(input: &Input) -> usize {
    simulate_monkeys(input, 20, |worry| worry / 3)
}

pub fn part2(input: &Input) -> usize {
    let modulo: u64 = input.1.iter().map(|monkey| monkey.test).product();
    simulate_monkeys(input, 10000, |worry| worry % modulo)
}

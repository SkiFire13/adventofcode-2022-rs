#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Item>, Vec<Monkey>);

pub struct Item {
    worry: u64,
    monkey: usize,
}

pub struct Monkey {
    exp: u32,
    prod: u64,
    sum: u64,
    test: u64,
    true_idx: usize,
    false_idx: usize,
}

pub fn input_generator(input: &str) -> Input {
    let mut items = Vec::new();
    let monkeys = input
        .split("\n\n")
        .enumerate()
        .map(|(idx, lines)| {
            let (_, monkey_items, op, test, true_mon, false_mon) =
                lines.lines().collect_tuple().expect("Invalid input");
            let (_, monkey_items) = monkey_items.split_once(": ").expect("Invalid input");
            let monkey_items = monkey_items
                .split(", ")
                .map(|n| n.parse().expect("Invalid input"))
                .map(|worry| Item { worry, monkey: idx });
            items.extend(monkey_items);
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
            Monkey { exp, prod, sum, test, true_idx, false_idx }
        })
        .collect();
    (items, monkeys)
}

fn simulate_monkeys<const CACHE: bool>(
    input: &Input,
    rounds: usize,
    reduce: impl Fn(u64) -> u64,
) -> usize {
    let (items, monkeys) = input;
    let mut inspected = vec![0; monkeys.len()];

    let mut seen = HashMap::new();
    let mut inspected_sequence = Vec::new();

    for item in items {
        let Item { mut worry, mut monkey } = item;

        seen.clear();
        inspected_sequence.clear();
        let mut already_skipped = false;

        let mut round = 0;
        while round < rounds {
            if CACHE && !already_skipped {
                if let Some((cycled_round, cycled_idx)) =
                    seen.insert((monkey, worry), (round, inspected_sequence.len()))
                {
                    let steps = round - cycled_round;
                    let cycles = (rounds - round - 1) / steps;
                    inspected_sequence[cycled_idx..]
                        .iter()
                        .for_each(|&monkey| inspected[monkey] += cycles);
                    round += steps * cycles;
                    already_skipped = true;
                }
                inspected_sequence.push(monkey);
            }

            inspected[monkey] += 1;

            let Monkey { exp, prod, sum, test, true_idx, false_idx } = monkeys[monkey];
            let prev_monkey = monkey;

            worry = reduce(worry.pow(exp) * prod + sum);
            monkey = match worry % test {
                0 => true_idx,
                _ => false_idx,
            };
            round += (monkey < prev_monkey) as usize;
        }
    }

    inspected.sort_unstable();
    inspected[inspected.len() - 2] * inspected[inspected.len() - 1]
}

pub fn part1(input: &Input) -> usize {
    simulate_monkeys::<false>(input, 20, |worry| worry / 3)
}

pub fn part2(input: &Input) -> usize {
    let modulo: u64 = input.1.iter().map(|monkey| monkey.test).product();
    simulate_monkeys::<true>(input, 10000, |worry| worry % modulo)
}

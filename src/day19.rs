#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<BluePrint>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once("Blueprint ").expect("Invalid input");
            let (id, rest) = rest.split_once(":").expect("Invalid input");
            let (_, rest) = rest.split_once(" costs ").expect("Invalid input");
            let (ore_ore_cost, rest) = rest.split_once(" ore").expect("Invalid input");
            let (_, rest) = rest.split_once(" costs ").expect("Invalid input");
            let (clay_ore_cost, rest) = rest.split_once(" ore").expect("Invalid input");
            let (_, rest) = rest.split_once(" costs ").expect("Invalid input");
            let (obs_ore_cost, rest) = rest.split_once(" ore and ").expect("Invalid input");
            let (obs_clay_cost, rest) = rest.split_once(" clay").expect("Invalid input");
            let (_, rest) = rest.split_once(" costs ").expect("Invalid input");
            let (geode_ore_cost, rest) = rest.split_once(" ore and ").expect("Invalid input");
            let (geode_obs_cost, _) = rest.split_once(" obsidian.").expect("Invalid input");
            let id = id.parse().expect("Invalid input");
            let ore_ore_cost = ore_ore_cost.parse().expect("Invalid input");
            let clay_ore_cost = clay_ore_cost.parse().expect("Invalid input");
            let obs_ore_cost = obs_ore_cost.parse().expect("Invalid input");
            let obs_clay_cost = obs_clay_cost.parse().expect("Invalid input");
            let geode_ore_cost = geode_ore_cost.parse().expect("Invalid input");
            let geode_obs_cost = geode_obs_cost.parse().expect("Invalid input");
            BluePrint {
                id,
                costs: [
                    [ore_ore_cost, 0, 0, 0],
                    [clay_ore_cost, 0, 0, 0],
                    [obs_ore_cost, obs_clay_cost, 0, 0],
                    [geode_ore_cost, 0, geode_obs_cost, 0],
                ],
            }
        })
        .collect()
}

pub struct BluePrint {
    id: usize,
    costs: [[u16; 4]; 4],
}

fn solve(blueprint: &BluePrint, time: u16) -> usize {
    let &BluePrint { id: _, costs } = blueprint;

    let mut max_costs: [_; 4] = array::from_fn(|i| (0..4).map(|j| costs[j][i]).max().unwrap());
    max_costs[3] = u16::MAX;

    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Entry {
        upper_bound: u16,
        time: u16,
        ores: [u16; 4],
        robots: [u16; 4],
    }

    let mut initial = Entry { upper_bound: 1, time, ..Default::default() };
    initial.robots[0] = 1;
    let mut stack = Vec::from([initial]);
    let mut best = 0;

    while let Some(entry) = stack.pop() {
        let Entry { upper_bound, time, ores, robots } = entry;

        if upper_bound <= best {
            continue;
        }

        best = max(best, ores[3] + robots[3] * time);

        for i in 0..4 {
            if robots[i] == max_costs[i] {
                continue;
            }

            let wait = (0..3)
                .map(|j| match () {
                    _ if costs[i][j] <= ores[j] => 0,
                    _ if robots[j] == 0 => time,
                    _ => 1 + (costs[i][j] - ores[j] - 1) / robots[j],
                })
                .max()
                .unwrap();
            if wait + 1 >= time {
                continue;
            }

            let mut new_entry = entry;
            for j in 0..4 {
                new_entry.ores[j] = new_entry.ores[j] + robots[j] * (wait + 1) - costs[i][j];
                (robots[j] >= max_costs[j]).then(|| new_entry.ores[j] = max_costs[j]);
            }
            new_entry.time -= wait + 1;
            new_entry.robots[i] += 1;
            new_entry.upper_bound = {
                let mut ores = [new_entry.ores; 4];
                let mut robots = new_entry.robots;

                for _ in 0..new_entry.time {
                    let mut new_ores = ores;
                    for i in 0..4 {
                        for j in 0..4 {
                            new_ores[i][j] += robots[j];
                        }
                    }
                    for i in 0..4 {
                        if (0..3).all(|j| ores[i][j] >= costs[i][j]) {
                            (0..3).for_each(|j| new_ores[i][j] -= costs[i][j]);
                            robots[i] += 1;
                        }
                    }
                    ores = new_ores;
                }

                ores[0][3]
            };

            if new_entry.upper_bound > best {
                stack.push(new_entry);
            }
        }
    }

    best as usize
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|blueprint| blueprint.id * solve(blueprint, 24))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input[..3]
        .iter()
        .map(|blueprint| solve(blueprint, 32))
        .product()
}

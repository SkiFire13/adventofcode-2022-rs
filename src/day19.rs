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
                ore_ore_cost,
                clay_ore_cost,
                obs_ore_cost,
                obs_clay_cost,
                geode_ore_cost,
                geode_obs_cost,
            }
        })
        .collect()
}

pub struct BluePrint {
    id: usize,
    ore_ore_cost: u16,
    clay_ore_cost: u16,
    obs_ore_cost: u16,
    obs_clay_cost: u16,
    geode_ore_cost: u16,
    geode_obs_cost: u16,
}

fn solve(blueprint: &BluePrint, time: u16) -> usize {
    let &BluePrint {
        id: _,
        ore_ore_cost,
        clay_ore_cost,
        obs_ore_cost,
        obs_clay_cost,
        geode_ore_cost,
        geode_obs_cost,
    } = blueprint;

    let max_ore_cost = max(clay_ore_cost, max(obs_ore_cost, geode_obs_cost));
    let max_clay_cost = obs_clay_cost;
    let max_obs_cost = geode_obs_cost;

    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Entry {
        upper_bound: u16,
        geode: u16,
        time: u16,
        obs_miners: u16,
        clay_miners: u16,
        ore_miners: u16,
        ore: u16,
        clay: u16,
        obs: u16,
    }

    let initial = Entry { upper_bound: 1, time, ore_miners: 1, ..Default::default() };
    let mut stack = Vec::from([initial]);
    let mut seen = HashSet::new();

    let mut best = 0;

    while let Some(entry) = stack.pop() {
        if entry.upper_bound as usize <= best || !seen.insert(entry) {
            continue;
        }

        if entry.time == 0
            || (entry.ore >= geode_ore_cost
                && entry.obs >= geode_obs_cost
                && entry.ore_miners >= geode_ore_cost
                && entry.obs_miners >= geode_obs_cost)
        {
            best = max(best, entry.upper_bound as usize);
            continue;
        }

        let mut new_entry = entry;
        new_entry.ore += entry.ore_miners;
        new_entry.clay += entry.clay_miners;
        new_entry.obs += entry.obs_miners;
        (entry.ore_miners == max_ore_cost).then(|| new_entry.ore = max_ore_cost);
        (entry.clay_miners == max_clay_cost).then(|| new_entry.clay = max_clay_cost);
        (entry.obs_miners == max_obs_cost).then(|| new_entry.obs = max_obs_cost);

        new_entry.time -= 1;

        let add_upper_bound = |miners, limiting_factors: &[_]| {
            let time = new_entry.time;
            let &miners_upper_bound = limiting_factors.iter().chain([&time]).min().unwrap();
            miners * time
                + miners_upper_bound * miners_upper_bound.saturating_sub(1) / 2
                + miners_upper_bound * (time - miners_upper_bound)
        };

        let ore_upper_bound = new_entry.ore + add_upper_bound(new_entry.ore_miners, &[]);
        let clay_upper_bound = new_entry.clay
            + add_upper_bound(new_entry.clay_miners, &[ore_upper_bound / clay_ore_cost]);
        let obs_limiting = &[
            ore_upper_bound / obs_ore_cost,
            clay_upper_bound / obs_clay_cost,
        ];
        let obs_upper_bound = new_entry.obs + add_upper_bound(new_entry.obs_miners, obs_limiting);
        let geode_limiting = &[
            ore_upper_bound / geode_ore_cost,
            obs_upper_bound / geode_obs_cost,
        ];
        new_entry.upper_bound = new_entry.geode + add_upper_bound(0, geode_limiting);

        if new_entry.time == 0
            || entry.ore < max_ore_cost
            || entry.clay < max_clay_cost
            || entry.obs < max_obs_cost
        {
            stack.push(new_entry)
        };

        if new_entry.time > 0 {
            if entry.ore >= ore_ore_cost && entry.ore_miners < max_ore_cost {
                let mut new_entry = new_entry;
                new_entry.ore_miners += 1;
                new_entry.ore -= ore_ore_cost;
                stack.push(new_entry);
            }

            if entry.ore >= clay_ore_cost && entry.clay_miners < max_clay_cost {
                let mut new_entry = new_entry;
                new_entry.clay_miners += 1;
                new_entry.ore -= clay_ore_cost;
                stack.push(new_entry);
            }

            if entry.ore >= obs_ore_cost
                && entry.clay >= obs_clay_cost
                && entry.obs_miners < max_obs_cost
            {
                let mut new_entry = new_entry;
                new_entry.obs_miners += 1;
                new_entry.ore -= obs_ore_cost;
                new_entry.clay -= obs_clay_cost;
                stack.push(new_entry);
            }

            if entry.ore >= geode_ore_cost && entry.obs >= geode_obs_cost {
                let mut new_entry = new_entry;
                new_entry.upper_bound += new_entry.time;
                new_entry.geode += new_entry.time;
                new_entry.ore -= geode_ore_cost;
                new_entry.obs -= geode_obs_cost;
                stack.push(new_entry);
            }
        }
    }

    best
}

pub fn part1(input: &Input) -> usize {
    input
        .par_iter()
        .map(|blueprint| blueprint.id * solve(blueprint, 24))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input[..3]
        .par_iter()
        .map(|blueprint| solve(blueprint, 32))
        .product()
}

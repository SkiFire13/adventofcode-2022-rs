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

        new_entry.upper_bound = {
            let mut ore_for_ore = new_entry.ore;
            let mut ore_for_clay = new_entry.ore;
            let mut ore_for_obs = new_entry.ore;
            let mut ore_for_geode = new_entry.ore;
            let mut ore_miners = new_entry.ore_miners;
            let mut clay_for_obs = new_entry.clay;
            let mut clay_miners = new_entry.clay_miners;
            let mut obs_for_geode = new_entry.obs;
            let mut obs_miners = new_entry.obs_miners;
            let mut geode = new_entry.geode;
            for i in (0..new_entry.time).rev() {
                ore_for_ore += ore_miners;
                ore_for_clay += ore_miners;
                ore_for_obs += ore_miners;
                ore_for_geode += ore_miners;
                clay_for_obs += clay_miners;
                obs_for_geode += obs_miners;
                if ore_for_ore >= ore_ore_cost {
                    ore_for_ore -= ore_ore_cost;
                    ore_miners += 1;
                }
                if ore_for_clay >= clay_ore_cost {
                    ore_for_clay -= clay_ore_cost;
                    clay_miners += 1;
                }
                if ore_for_obs >= obs_ore_cost && clay_for_obs >= obs_clay_cost {
                    ore_for_obs -= obs_ore_cost;
                    clay_for_obs -= obs_clay_cost;
                    obs_miners += 1;
                }
                if ore_for_geode >= geode_ore_cost && obs_for_geode >= geode_obs_cost {
                    ore_for_geode -= geode_ore_cost;
                    obs_for_geode -= geode_obs_cost;
                    geode += i;
                }
            }
            geode
        };

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

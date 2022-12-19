#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize, usize, (usize, usize), (usize, usize))>;

pub fn input_generator(input: &str) -> Input {
    //     "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once("Blueprint ").unwrap();
            let (blueprint, rest) = rest.split_once(": Each ore robot costs ").unwrap();
            let (ore_cost, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();
            let (clay_cost, rest) = rest.split_once(" ore. Each obsidian robot costs ").unwrap();
            let (ob_ore_cost, rest) = rest.split_once(" ore and ").unwrap();
            let (ob_clay_cost, rest) = rest.split_once(" clay. Each geode robot costs ").unwrap();
            let (geode_ore_cost, rest) = rest.split_once(" ore and ").unwrap();
            let (geode_ob_cost, _) = rest.split_once(" obsidian.").unwrap();
            let blueprint = blueprint.parse().unwrap();
            let ore_cost = ore_cost.parse().unwrap();
            let clay_cost = clay_cost.parse().unwrap();
            let ob_ore_cost = ob_ore_cost.parse().unwrap();
            let ob_clay_cost = ob_clay_cost.parse().unwrap();
            let geode_ore_cost = geode_ore_cost.parse().unwrap();
            let geode_ob_cost = geode_ob_cost.parse().unwrap();
            (
                blueprint,
                ore_cost,
                clay_cost,
                (ob_ore_cost, ob_clay_cost),
                (geode_ore_cost, geode_ob_cost),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    return 0;
    input
        .par_iter()
        .map(
            |&(
                blueprint,
                ore_ore_cost,
                clay_ore_cost,
                (ob_ore_cost, ob_clay_cost),
                (geode_ore_cost, geode_ob_cost),
            )| {
                let mut queue = BinaryHeap::from([(22, 0, 24, 0, 0, 0, 1, 0, 0, 0)]);
                let mut seen = HashSet::new();

                while let Some((
                    _,
                    geode,
                    time,
                    geode_miners,
                    ob_miners,
                    clay_miners,
                    ore_miners,
                    ore,
                    clay,
                    obs,
                )) = queue.pop()
                {
                    if time == 0 {
                        return dbg!(dbg!(geode) * blueprint);
                    }

                    if !seen.insert((
                        geode,
                        time,
                        geode_miners,
                        ob_miners,
                        clay_miners,
                        ore_miners,
                        ore,
                        clay,
                        obs,
                    )) {
                        continue;
                    }

                    let new_ore = ore + ore_miners;
                    let new_clay = clay + clay_miners;
                    let new_obs = obs + ob_miners;
                    let new_geode = geode + geode_miners;

                    queue.push((
                        new_geode + (geode_miners + time - 2) * (time - 2),
                        new_geode,
                        time - 1,
                        geode_miners,
                        ob_miners,
                        clay_miners,
                        ore_miners,
                        new_ore,
                        new_clay,
                        new_obs,
                    ));

                    if time > 1 {
                        if ore >= ore_ore_cost {
                            queue.push((
                                new_geode + (geode_miners + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners,
                                ob_miners,
                                clay_miners,
                                ore_miners + 1,
                                new_ore - ore_ore_cost,
                                new_clay,
                                new_obs,
                            ));
                        }

                        if ore >= clay_ore_cost {
                            queue.push((
                                new_geode + (geode_miners + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners,
                                ob_miners,
                                clay_miners + 1,
                                ore_miners,
                                new_ore - clay_ore_cost,
                                new_clay,
                                new_obs,
                            ));
                        }

                        if ore >= ob_ore_cost && clay >= ob_clay_cost {
                            queue.push((
                                new_geode + (geode_miners + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners,
                                ob_miners + 1,
                                clay_miners,
                                ore_miners,
                                new_ore - ob_ore_cost,
                                new_clay - ob_clay_cost,
                                new_obs,
                            ));
                        }

                        if ore >= geode_ore_cost && obs >= geode_ob_cost {
                            queue.push((
                                new_geode + 1 + (geode_miners + 1 + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners + 1,
                                ob_miners,
                                clay_miners,
                                ore_miners,
                                new_ore - geode_ore_cost,
                                new_clay,
                                new_obs - geode_ob_cost,
                            ));
                        }
                    }
                }

                unreachable!()
            },
        )
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input[..3]
        .par_iter()
        .map(
            |&(
                _,
                ore_ore_cost,
                clay_ore_cost,
                (ob_ore_cost, ob_clay_cost),
                (geode_ore_cost, geode_ob_cost),
            )| {
                let mut queue = BinaryHeap::from([(30, 0, 32, 0, 0, 0, 1, 0, 0, 0)]);
                let mut seen = HashSet::new();

                while let Some((
                    _,
                    geode,
                    time,
                    geode_miners,
                    ob_miners,
                    clay_miners,
                    ore_miners,
                    ore,
                    clay,
                    obs,
                )) = queue.pop()
                {
                    if time == 0 {
                        return dbg!(geode);
                    }

                    if !seen.insert((
                        geode,
                        time,
                        geode_miners,
                        ob_miners,
                        clay_miners,
                        ore_miners,
                        ore,
                        clay,
                        obs,
                    )) {
                        continue;
                    }

                    let new_ore = ore + ore_miners;
                    let new_clay = clay + clay_miners;
                    let new_obs = obs + ob_miners;
                    let new_geode = geode + geode_miners;

                    queue.push((
                        new_geode + (geode_miners + time - 2) * (time - 2),
                        new_geode,
                        time - 1,
                        geode_miners,
                        ob_miners,
                        clay_miners,
                        ore_miners,
                        new_ore,
                        new_clay,
                        new_obs,
                    ));

                    if time > 1 {
                        if ore >= ore_ore_cost {
                            queue.push((
                                new_geode + (geode_miners + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners,
                                ob_miners,
                                clay_miners,
                                ore_miners + 1,
                                new_ore - ore_ore_cost,
                                new_clay,
                                new_obs,
                            ));
                        }

                        if ore >= clay_ore_cost {
                            queue.push((
                                new_geode + (geode_miners + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners,
                                ob_miners,
                                clay_miners + 1,
                                ore_miners,
                                new_ore - clay_ore_cost,
                                new_clay,
                                new_obs,
                            ));
                        }

                        if ore >= ob_ore_cost && clay >= ob_clay_cost {
                            queue.push((
                                new_geode + (geode_miners + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners,
                                ob_miners + 1,
                                clay_miners,
                                ore_miners,
                                new_ore - ob_ore_cost,
                                new_clay - ob_clay_cost,
                                new_obs,
                            ));
                        }

                        if ore >= geode_ore_cost && obs >= geode_ob_cost {
                            queue.push((
                                new_geode + 1 + (geode_miners + 1 + time - 2) * (time - 2),
                                new_geode,
                                time - 1,
                                geode_miners + 1,
                                ob_miners,
                                clay_miners,
                                ore_miners,
                                new_ore - geode_ore_cost,
                                new_clay,
                                new_obs - geode_ob_cost,
                            ));
                        }
                    }
                }

                unreachable!()
            },
        )
        .product()
}

#[allow(unused_imports)]
use super::prelude::*;
type Input = HashMap<usize, (usize, Vec<usize>)>;

pub fn input_generator(input: &str) -> Input {
    let mut ids = HashMap::from([("AA", 0)]);
    let mut id = |s| {
        let next_id = ids.len();
        *ids.entry(s).or_insert(next_id)
    };
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once("Valve ").unwrap();
            let (name, rest) = rest.split_once(" has flow rate=").unwrap();
            let (flow_rate, rest) = rest
                .split_once("; tunnels lead to valves ")
                .or_else(|| rest.split_once("; tunnel leads to valve "))
                .unwrap();
            let valves = rest.split(", ").map(&mut id).collect();
            let flow_rate = flow_rate.parse().unwrap();
            (id(name), (flow_rate, valves))
        })
        .collect()
}

fn distances(input: &Input) -> HashMap<usize, Vec<(usize, usize)>> {
    let nonzero = input
        .iter()
        .filter(|(_, &(flow, _))| flow != 0)
        .map(|(&k, _)| k)
        .collect::<HashSet<_>>();

    nonzero
        .iter()
        .chain([&0])
        .map(|&k| {
            let mut queue = BinaryHeap::from([(Reverse(0), k)]);
            let mut distances = HashMap::new();

            while let Some((Reverse(d), k)) = queue.pop() {
                if distances.contains_key(&k) {
                    continue;
                }
                distances.insert(k, d);
                queue.extend(input[&k].1.iter().map(|&next| (Reverse(d + 1), next)));
            }

            let mut distances = Vec::from_iter(distances);
            distances.retain(|&(next, _)| next != k && nonzero.contains(&next));
            distances.sort_by_key(|&(_, d)| d);

            (k, distances)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    #[derive(Default)]
    struct SolveRecData {
        curr: usize,
        curr_pressure: usize,
        tot_pressure: usize,
        time: usize,
    }

    fn solve_rec(
        opened: &mut HashSet<usize>,
        data: SolveRecData,
        input: &Input,
        distances: &HashMap<usize, Vec<(usize, usize)>>,
    ) -> usize {
        distances[&data.curr]
            .iter()
            .filter_map(|&(next, time_needed)| {
                let remaining_time = data.time.checked_sub(time_needed + 1)?;

                if !opened.insert(next) {
                    return None;
                }

                let flow_rate = input[&next].0;
                let new_data = SolveRecData {
                    curr: next,
                    tot_pressure: data.tot_pressure + data.curr_pressure * (time_needed + 1),
                    curr_pressure: data.curr_pressure + flow_rate,
                    time: remaining_time,
                };
                let best_rec = solve_rec(opened, new_data, input, distances);
                opened.remove(&next);
                Some(best_rec)
            })
            .chain([data.tot_pressure + data.curr_pressure * data.time])
            .max()
            .unwrap()
    }

    let data = SolveRecData { time: 30, ..Default::default() };
    solve_rec(&mut HashSet::new(), data, input, &distances(input))
}

pub fn part2(input: &Input) -> usize {
    #[derive(Default)]
    struct SolveRecData {
        curr0: usize,
        curr0_remaining: usize,
        curr1: usize,
        curr1_remaining: usize,
        curr_pressure: usize,
        tot_pressure: usize,
        time: usize,
    }

    fn solve_rec(
        opened: &mut HashSet<usize>,
        mut data: SolveRecData,
        input: &Input,
        distances: &HashMap<usize, Vec<(usize, usize)>>,
    ) -> usize {
        if data.time == 0 {
            return data.tot_pressure;
        }

        if data.curr0_remaining > data.curr1_remaining {
            std::mem::swap(&mut data.curr0, &mut data.curr1);
            std::mem::swap(&mut data.curr0_remaining, &mut data.curr1_remaining);
        }

        data.tot_pressure += data.curr_pressure * data.curr0_remaining;
        data.time -= data.curr0_remaining;
        data.curr1_remaining -= data.curr0_remaining;
        data.curr_pressure += input[&data.curr0].0;

        let new_data = SolveRecData { curr0_remaining: data.time, ..data };
        let best_curr0_stop = solve_rec(opened, new_data, input, distances);

        distances[&data.curr0]
            .iter()
            .filter_map(|&(next, time_needed)| {
                let curr0_remaining = time_needed + 1;
                if curr0_remaining > data.time || !opened.insert(next) {
                    return None;
                }

                let new_data = SolveRecData { curr0: next, curr0_remaining, ..data };
                let best_rec = solve_rec(opened, new_data, input, distances);
                opened.remove(&next);
                Some(best_rec)
            })
            .chain([best_curr0_stop])
            .max()
            .unwrap()
    }

    let data = SolveRecData { time: 26, ..Default::default() };
    solve_rec(&mut HashSet::new(), data, input, &distances(input))
}

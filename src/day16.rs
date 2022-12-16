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
            let (name, rest) = line[6..].split_once(' ').expect("Invalid input");
            let (_, rest) = rest.split_once('=').expect("Invalid input");
            let (flow_rate, rest) = rest.split_once(';').expect("Invalid input");
            let (_, rest) = rest.split_once("valve").expect("Invalid input");
            let rest = rest.trim_start_matches('s').trim_start();
            let valves = rest.split(", ").map(&mut id).collect();
            let flow_rate = flow_rate.parse().expect("Invalid input");
            (id(name), (flow_rate, valves))
        })
        .collect()
}

fn nonzero(input: &Input) -> Vec<usize> {
    let mut nonzero = vec![usize::MAX; input.len()];
    input
        .iter()
        .filter(|&(_, &(flow, _))| flow != 0)
        .enumerate()
        .for_each(|(nzidx, (&id, _))| nonzero[id] = nzidx);
    nonzero
}

fn distances(input: &Input, nonzero: &[usize]) -> HashMap<usize, Vec<(usize, (u64, usize))>> {
    (0..input.len())
        .filter(|&idx| idx == 0 || nonzero[idx] != usize::MAX)
        .map(|k| {
            let mut queue = VecDeque::from([(k, 0, 0)]);
            let mut distances = HashMap::new();

            while let Some((k, mut filter, d)) = queue.pop_front() {
                if !distances.contains_key(&k) {
                    distances.insert(k, (filter, d));
                    filter |= (nonzero[k] != usize::MAX)
                        .then(|| 1 << nonzero[k])
                        .unwrap_or(0);
                    queue.extend(input[&k].1.iter().map(|&next| (next, filter, d + 1)));
                }
            }

            let mut distances = Vec::from_iter(distances);
            distances.retain(|&(next, _)| next != k && nonzero[next] != usize::MAX);
            distances.sort_by_key(|&(_, v)| v);

            (k, distances)
        })
        .collect()
}

#[derive(Default)]
struct SolveData {
    node0: usize,
    remaining0: usize,
    node1: usize,
    remaining1: usize,
    pressure: usize,
    time: usize,
    opened: u64,
}

fn solve(input: &Input, data: SolveData) -> usize {
    fn solve_rec(
        mut data: SolveData,
        mut best: usize,
        max_flow: usize,
        input: &Input,
        nonzero: &[usize],
        distances: &HashMap<usize, Vec<(usize, (u64, usize))>>,
    ) -> usize {
        if data.remaining0 > data.remaining1 {
            std::mem::swap(&mut data.node0, &mut data.node1);
            std::mem::swap(&mut data.remaining0, &mut data.remaining1);
        }

        if data.remaining0 >= data.time {
            return data.pressure;
        }

        let flow = input[&data.node0].0;
        data.time -= data.remaining0;
        data.remaining1 -= data.remaining0;
        data.pressure += data.time * flow;
        let max_flow = max_flow - flow;

        if data.pressure + data.time * max_flow <= best {
            return 0;
        }

        for &(next, (filter, time_needed)) in &distances[&data.node0] {
            if time_needed < data.time
                && data.opened & filter == filter
                && (data.opened & (1 << nonzero[next])) == 0
            {
                let remaining0 = time_needed + 1;
                let opened = data.opened | (1 << nonzero[next]);
                let new_data = SolveData { node0: next, remaining0, opened, ..data };

                let best_rec = solve_rec(new_data, best, max_flow, input, nonzero, distances);
                best = max(best, best_rec);
            }
        }

        let new_data = SolveData { remaining0: data.time, ..data };
        let best_stop = (new_data.remaining1 < data.time)
            .then(|| solve_rec(new_data, best, max_flow, input, nonzero, distances))
            .unwrap_or(data.pressure);

        max(best, best_stop)
    }

    let max_flow = input.values().map(|&(flow, _)| flow).sum();
    let nonzero = nonzero(input);
    let distances = distances(input, &nonzero);
    solve_rec(data, 0, max_flow, input, &nonzero, &distances)
}

pub fn part1(input: &Input) -> usize {
    let data = SolveData { time: 30, remaining1: 30, ..Default::default() };
    solve(input, data)
}

pub fn part2(input: &Input) -> usize {
    let data = SolveData { time: 26, ..Default::default() };
    solve(input, data)
}

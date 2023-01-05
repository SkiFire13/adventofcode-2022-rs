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
            let flow = flow_rate.parse().expect("Invalid input");
            (id(name), (flow, valves))
        })
        .collect()
}

fn distances(input: &Input) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut flows = vec![0];
    let mut nonzero = vec![usize::MAX; input.len()];
    nonzero[0] = 0;
    for (&node, &(flow, _)) in input {
        if flow != 0 {
            nonzero[node] = flows.len();
            flows.push(flow);
        }
    }

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut distances = vec![vec![usize::MAX; flows.len()]; flows.len()];
    for (raw_node, &nz_node) in nonzero.iter().enumerate() {
        if nz_node != usize::MAX {
            let distances = &mut distances[nz_node];
            seen.clear();
            queue.push_back((raw_node, 0));
            while let Some((node, d)) = queue.pop_front() {
                if seen.insert(node) {
                    if nonzero[node] != usize::MAX {
                        distances[nonzero[node]] = d;
                    }
                    queue.extend(input[&node].1.iter().map(|&next| (next, d + 1)));
                }
            }
        }
    }
    (distances, flows)
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct NodeData {
    upper_bound: usize,
    pressure: usize,
    node: usize,
    time: usize,
    node2: usize,
    time2: usize,
    opened: u64,
}

fn solve(input: &Input, time: usize, time2: usize) -> usize {
    let (distances, flows) = distances(input);
    let min_distances = (0..=max(time, time2))
        .map(|t| {
            distances
                .iter()
                .map(|distances| {
                    distances
                        .iter()
                        .copied()
                        .filter(|&d| d != 0)
                        .min()
                        .unwrap_or(usize::MAX - 1)
                })
                .enumerate()
                .filter(|&(id, d)| t > d + 1 && flows[id] != 0)
                .sorted_unstable_by_key(|&(id, d)| Reverse(flows[id] * t - (d + 1)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut initial = NodeData { time, time2, ..Default::default() };
    initial.opened = 1 << 0;
    initial.upper_bound = usize::MAX;

    let mut best = 0;
    let mut queue = BinaryHeap::from_iter([initial]);
    let mut seen = FxHashSet::default();
    while let Some(mut data) = queue.pop() {
        best = max(best, data.pressure);

        if data.upper_bound <= best {
            break;
        }

        if data.time < data.time2 {
            swap(&mut data.time, &mut data.time2);
            swap(&mut data.node, &mut data.node2);
        }

        if !seen.insert(NodeData { upper_bound: 0, pressure: 0, ..data }) {
            continue;
        }

        let upper_bound = |data: NodeData| {
            let mut upper_bound = data.pressure;
            let mut opened = data.opened;
            let mut time = max(data.time, data.time2);
            let mut time2 = min(data.time, data.time2);
            'outer: loop {
                for &(id, d) in &min_distances[time] {
                    if opened & (1 << id) == 0 {
                        opened |= 1 << id;
                        time -= d + 1;
                        upper_bound += flows[id] * time;
                        (time, time2) = (max(time, time2), min(time, time2));
                        continue 'outer;
                    }
                }
                return upper_bound;
            }
        };

        for (node, &time_needed) in distances[data.node].iter().enumerate() {
            if time_needed < data.time && (data.opened & (1 << node)) == 0 {
                let time = data.time - (time_needed + 1);
                let mut data = NodeData { node, time, ..data };
                data.pressure += time * flows[node];
                data.opened |= 1 << node;
                data.upper_bound = upper_bound(data);
                if data.upper_bound > best {
                    queue.push(data);
                }
            }
        }

        if data.time2 != 0 {
            let mut data = NodeData { time: 0, ..data };
            data.upper_bound = upper_bound(data);
            if data.upper_bound > best {
                queue.push(data);
            }
        }
    }

    best
}

pub fn part1(input: &Input) -> usize {
    solve(input, 30, 0)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 26, 26)
}

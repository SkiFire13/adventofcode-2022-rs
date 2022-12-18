#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(isize, isize, isize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line.split(',').collect_tuple().expect("Invalid input");
            let x = x.parse().expect("Invalid input");
            let y = y.parse().expect("Invalid input");
            let z = z.parse().expect("Invalid input");
            (x, y, z)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let set: HashSet<_> = HashSet::from_iter(input.iter().copied());

    input
        .iter()
        .flat_map(|&(x, y, z)| {
            [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
        })
        .filter(|p| !set.contains(p))
        .count()
}

pub fn part2(input: &Input) -> usize {
    let minmaxx = input
        .iter()
        .map(|&(x, y, z)| ((y, z), x))
        .into_grouping_map()
        .minmax();
    let minmaxy = input
        .iter()
        .map(|&(x, y, z)| ((x, z), y))
        .into_grouping_map()
        .minmax();
    let minmaxz = input
        .iter()
        .map(|&(x, y, z)| ((x, y), z))
        .into_grouping_map()
        .minmax();
    let (minx, maxx) = input
        .iter()
        .map(|&(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = input
        .iter()
        .map(|&(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap();
    let (minz, maxz) = input
        .iter()
        .map(|&(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap();

    let mut set: HashSet<_> = HashSet::from_iter(input.iter().copied());

    for x in minx..=maxx {
        for y in miny..=maxy {
            for z in minz..=maxz {
                if set.contains(&(x, y, z)) {
                    continue;
                }
                use itertools::MinMaxResult;
                let mut fill = true;
                if let Some(&minmaxx) = minmaxx.get(&(y, z)) {
                    fill &= match minmaxx {
                        MinMaxResult::NoElements => panic!(),
                        MinMaxResult::OneElement(minmaxx) => x == minmaxx,
                        MinMaxResult::MinMax(minx, maxx) => minx <= x && x <= maxx,
                    };
                }
                if let Some(&minmaxy) = minmaxy.get(&(x, z)) {
                    fill &= match minmaxy {
                        MinMaxResult::NoElements => panic!(),
                        MinMaxResult::OneElement(minmaxy) => y == minmaxy,
                        MinMaxResult::MinMax(miny, maxy) => miny <= y && y <= maxy,
                    };
                }
                if let Some(&minmaxz) = minmaxz.get(&(x, y)) {
                    fill &= match minmaxz {
                        MinMaxResult::NoElements => panic!(),
                        MinMaxResult::OneElement(minmaxz) => z == minmaxz,
                        MinMaxResult::MinMax(minz, maxz) => minz <= z && z <= maxz,
                    };
                }
                if fill {
                    set.insert((x, y, z));
                }
            }
        }
    }

    input
        .iter()
        .flat_map(|&(x, y, z)| {
            [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
        })
        .filter(|p| !set.contains(p))
        .count()
}

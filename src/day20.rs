#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<i64>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

fn solve(input: &Input, factor: i64, iters: usize) -> i64 {
    let len = input.len();

    let bucket_size = num::integer::sqrt(len);
    let mut buckets = (0..len)
        .batching(|it| Some(it.take(bucket_size).collect::<Vec<_>>()))
        .take_while(|v| !v.is_empty())
        .collect::<Vec<_>>();

    let mut idxs = (0..len).map(|i| i / bucket_size).collect::<Vec<_>>();

    let get = |buckets: &[Vec<usize>], mut bucket: usize, pos, offset| {
        let mut offset = pos + offset;
        if offset < len / 2 {
            while offset > buckets[bucket].len() {
                offset -= buckets[bucket].len();
                bucket = (bucket + 1) % buckets.len();
            }
            (bucket, offset)
        } else {
            offset = len - 1 - (offset - buckets[bucket].len());
            while offset >= buckets[bucket].len() {
                offset -= buckets[bucket].len();
                bucket = bucket.checked_sub(1).unwrap_or(buckets.len() - 1);
            }
            (bucket, buckets[bucket].len() - offset)
        }
    };

    for _ in 0..iters {
        for node in 0..len {
            let shift = (input[node] * factor).rem_euclid(len as i64 - 1) as usize;
            let bucket = idxs[node];
            let pos = buckets[bucket].iter().position(|&n| n == node).unwrap();
            buckets[bucket].remove(pos);
            let (bucket, offset) = get(&buckets, bucket, pos, shift);
            buckets[bucket].insert(offset, node);
            idxs[node] = bucket;
        }
    }

    let zero = input.iter().position(|&n| n == 0).expect("Invalid input");
    let pos = buckets[idxs[zero]].iter().position(|&n| n == zero).unwrap();
    let (bucket1000, offset1000) = get(&buckets, idxs[zero], pos, 1000);
    let (bucket2000, offset2000) = get(&buckets, bucket1000, offset1000, 1000);
    let (bucket3000, offset3000) = get(&buckets, bucket2000, offset2000, 1000);
    let value = |bucket: usize, offset| input[buckets[bucket][offset]] * factor;
    value(bucket1000, offset1000) + value(bucket2000, offset2000) + value(bucket3000, offset3000)
}

pub fn part1(input: &Input) -> i64 {
    solve(input, 1, 1)
}

pub fn part2(input: &Input) -> i64 {
    solve(input, 811589153, 10)
}

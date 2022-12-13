#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Packet>;

pub enum Packet {
    Int(u8),
    Seq(Vec<Packet>),
}

fn parse_list_elem(input: &str) -> Packet {
    fn parse_inner(input: &mut &[u8]) -> Packet {
        let mut seq = Vec::new();
        let mut acc = None;
        loop {
            match eat_copy(input) {
                b'[' => seq.push(parse_inner(input)),
                b @ b'0'..=b'9' => acc = Some(acc.unwrap_or(0) * 10 + b - b'0'),
                b',' => seq.extend(acc.take().map(Packet::Int)),
                b']' => break,
                _ => panic!("Invalid input"),
            }
        }
        seq.extend(acc.take().map(Packet::Int));
        Packet::Seq(seq)
    }
    parse_inner(&mut input[1..].as_bytes())
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_list_elem)
        .collect()
}

fn cmp(p1: &Packet, p2: &Packet) -> Ordering {
    use std::slice::from_ref as slice_from_ref;
    let (s1, s2) = match (p1, p2) {
        (Packet::Seq(l1), Packet::Seq(l2)) => (&l1[..], &l2[..]),
        (Packet::Seq(l1), Packet::Int(_)) => (&l1[..], slice_from_ref(p2)),
        (Packet::Int(_), Packet::Seq(l2)) => (slice_from_ref(p1), &l2[..]),
        (Packet::Int(i1), Packet::Int(i2)) => return u8::cmp(&i1, &i2),
    };
    iter::zip(s1, s2)
        .map(|(p1, p2)| cmp(p1, p2))
        .find(|ord| ord.is_ne())
        .unwrap_or(usize::cmp(&s1.len(), &s2.len()))
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .tuples()
        .enumerate()
        .filter(|&(_, (p1, p2))| cmp(p1, p2).is_lt())
        .map(|(idx, _)| idx + 1)
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let (l, b) = input.iter().fold((1, input.len() + 2), |(l, u), p| {
        let lower_than_2 = cmp(p, &Packet::Int(2)).is_lt();
        let bigger_than_6 = !lower_than_2 && cmp(p, &Packet::Int(6)).is_gt();
        (l + lower_than_2 as usize, u - bigger_than_6 as usize)
    });
    l * b
}

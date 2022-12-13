#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<Packet>>;

#[derive(Copy, Clone)]
pub enum Packet {
    Int(u8),
    Seq(usize),
}

fn parse_list_elem(s: &str) -> Vec<Packet> {
    fn parse_inner(input: &mut &[u8], output: &mut Vec<Packet>) {
        let head_idx = output.len();
        output.push(Packet::Seq(0));
        let mut acc = None;
        loop {
            match eat_copy(input) {
                b'[' => parse_inner(input, output),
                b @ b'0'..=b'9' => acc = Some(acc.unwrap_or(0) * 10 + b - b'0'),
                b',' => output.extend(acc.take().map(Packet::Int)),
                b']' => break,
                _ => panic!("Invalid input"),
            }
        }
        output.extend(acc.take().map(Packet::Int));
        output[head_idx] = Packet::Seq(output.len() - head_idx - 1);
    }
    let mut output = Vec::with_capacity(s.len());
    parse_inner(&mut s[1..].as_bytes(), &mut output);
    output
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_list_elem)
        .collect()
}

fn cmp(p1: &[Packet], p2: &[Packet]) -> Ordering {
    use std::slice::from_ref as head_seq;
    use Packet::*;
    let &[h1, ref t1 @ ..] = p1 else { return usize::cmp(&0, &p2.len()) };
    let &[h2, ref t2 @ ..] = p2 else { return Ordering::Greater };
    match (h1, h2) {
        (Int(n1), Int(n2)) => u8::cmp(&n1, &n2).then_with(|| cmp(t1, t2)),
        (Int(_), Seq(l)) => cmp(head_seq(&h1), &t2[..l]).then_with(|| cmp(t1, &t2[l..])),
        (Seq(l), Int(_)) => cmp(&t1[..l], head_seq(&h2)).then_with(|| cmp(&t1[l..], t2)),
        (Seq(l1), Seq(l2)) => cmp(&t1[..l1], &t2[..l2]).then_with(|| cmp(&t1[l1..], &t2[l2..])),
    }
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
        let lower_than_2 = cmp(p, &[Packet::Int(2)]).is_lt();
        let bigger_than_6 = !lower_than_2 && cmp(p, &[Packet::Int(6)]).is_gt();
        (l + lower_than_2 as usize, u - bigger_than_6 as usize)
    });
    l * b
}

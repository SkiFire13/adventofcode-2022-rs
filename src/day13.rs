#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<[ListElem; 2]>;

pub enum ListElem {
    Int(u8),
    List(Vec<ListElem>),
}

fn parse_list_elem(s: &str) -> ListElem {
    fn parse_inner(s: &mut &[u8]) -> ListElem {
        match eat_copy(s) {
            b'[' => {
                let mut l = Vec::new();
                if s[0] != b']' {
                    l.push(parse_inner(s));
                }
                while eat_copy(s) == b',' {
                    l.push(parse_inner(s));
                }
                ListElem::List(l)
            }
            b => ListElem::Int(
                eat_while(s, u8::is_ascii_digit)
                    .iter()
                    .fold(b - b'0', |acc, b| acc * 10 + b - b'0'),
            ),
        }
    }
    parse_inner(&mut s.as_bytes())
}

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|line| {
            let (l1, l2) = line.split_once('\n').expect("Invalid input");
            [parse_list_elem(l1), parse_list_elem(l2)]
        })
        .collect()
}

impl PartialEq for ListElem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for ListElem {}
impl PartialOrd for ListElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListElem {
    fn cmp(&self, other: &ListElem) -> Ordering {
        use std::slice::from_ref as slice_from_ref;
        let (l1, l2) = (self, other);
        let (l1, l2) = match (l1, l2) {
            (ListElem::List(l1), ListElem::List(l2)) => (&l1[..], &l2[..]),
            (ListElem::List(l1), ListElem::Int(_)) => (&l1[..], slice_from_ref(l2)),
            (ListElem::Int(_), ListElem::List(l2)) => (slice_from_ref(l1), &l2[..]),
            (ListElem::Int(i1), ListElem::Int(i2)) => return i1.cmp(&i2),
        };
        l1.cmp(l2)
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|&(_, [l1, l2])| l1 <= l2)
        .map(|(idx, _)| idx + 1)
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut packets = input.iter().flatten().collect::<Vec<_>>();
    let p1 = ListElem::Int(2);
    let p2 = ListElem::Int(6);
    packets.extend([&p1, &p2]);
    packets.sort_unstable();
    let i1 = packets.binary_search(&&p1).unwrap();
    let i2 = packets.binary_search(&&p2).unwrap();
    (i1 + 1) * (i2 + 1)
}

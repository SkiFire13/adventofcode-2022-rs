#[allow(unused_imports)]
use super::prelude::*;
type Input = VecDeque<i64>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Copy, Clone)]
struct Node {
    value: i64,
    count: usize,
    left: NodeIdx,
    right: NodeIdx,
    parent: NodeIdx,
}

type NodeIdx = usize;
const NULL: NodeIdx = usize::MAX;

struct Treap<'a> {
    nodes: &'a mut [Node],
    root: NodeIdx,
}

impl Treap<'_> {
    fn update(&mut self, node: NodeIdx) {
        let Node { left, right, .. } = self.nodes[node];
        let mut count = 1;
        if let Some(l) = self.nodes.get_mut(left) {
            l.parent = node;
            count += l.count;
        }
        if let Some(r) = self.nodes.get_mut(right) {
            r.parent = node;
            count += r.count;
        }
        self.nodes[node].count = count;
    }

    fn split(&mut self, node: NodeIdx, rank: usize) -> (NodeIdx, NodeIdx) {
        let Some(&Node { left, right, .. })
            = self.nodes.get(node) else { return (NULL, NULL) };
        let left_count = self.nodes.get(left).map_or(0, |n| n.count);
        if rank <= left_count {
            let (ll, lr) = self.split(left, rank);
            self.nodes[node].left = lr;
            self.update(node);
            (ll, node)
        } else {
            let (rl, rr) = self.split(right, rank - left_count - 1);
            self.nodes[node].right = rl;
            self.update(node);
            (node, rr)
        }
    }

    fn merge(&mut self, left: NodeIdx, right: NodeIdx) -> NodeIdx {
        let Some(l) = self.nodes.get(left) else { return right };
        let Some(r) = self.nodes.get(right) else { return left };
        if l.value < r.value {
            self.nodes[left].right = self.merge(l.right, right);
            self.update(left);
            left
        } else {
            self.nodes[right].left = self.merge(left, r.left);
            self.update(right);
            right
        }
    }

    fn insert(&mut self, node: NodeIdx, rank: usize) {
        let n = &mut self.nodes[node];
        n.left = NULL;
        n.right = NULL;
        n.parent = NULL;
        n.count = 1;
        let (l, r) = self.split(self.root, rank);
        let lm = self.merge(l, node);
        self.root = self.merge(lm, r);
        self.nodes[self.root].parent = NULL;
    }

    fn remove(&mut self, node: NodeIdx) -> usize {
        let r = self.nodes[node];
        let mut rank = self.nodes.get(r.left).map_or(0, |n| n.count);
        let mut cur = r.parent;
        let mut prev = node;
        while let Some(c) = self.nodes.get_mut(cur) {
            let (l, r, p) = (c.left, c.right, c.parent);
            c.count -= 1;
            if prev == r {
                rank += 1 + self.nodes.get(l).map_or(0, |n| n.count);
            }
            (prev, cur) = (cur, p);
        }

        let merged = self.merge(r.left, r.right);
        if let Some(m) = self.nodes.get_mut(merged) {
            m.parent = r.parent;
        }
        if let Some(p) = self.nodes.get_mut(r.parent) {
            if p.left == node {
                p.left = merged;
            } else {
                p.right = merged;
            }
        } else {
            self.root = merged;
        }
        rank
    }

    fn rank(&self, node: NodeIdx) -> usize {
        let n = self.nodes[node];
        let mut rank = self.nodes.get(n.left).map_or(0, |n| n.count);
        let mut cur = n.parent;
        let mut prev = node;
        while let Some(c) = self.nodes.get(cur) {
            if prev == c.right {
                rank += 1 + self.nodes.get(c.left).map_or(0, |n| n.count);
            }
            (prev, cur) = (cur, c.parent);
        }
        rank
    }

    fn derank(&self, mut rank: usize) -> NodeIdx {
        let mut cur = self.root;
        while let Some(c) = self.nodes.get(cur) {
            let left_count = self.nodes.get(c.left).map_or(0, |n| n.count);
            match rank.cmp(&self.nodes.get(c.left).map_or(0, |n| n.count)) {
                Ordering::Less => cur = c.left,
                Ordering::Equal => return cur,
                Ordering::Greater => {
                    cur = c.right;
                    rank -= left_count + 1;
                }
            }
        }
        unreachable!()
    }
}

fn solve(len: usize, get: impl Fn(usize) -> i64, iter: impl Iterator<Item = usize>) -> i64 {
    let mut nodes = (0..len)
        .map(|i| Node {
            value: get(i),
            count: 1,
            left: NULL,
            right: NULL,
            parent: NULL,
        })
        .collect::<Vec<_>>();

    let zero = nodes
        .iter()
        .position(|n| n.value == 0)
        .expect("Invalid input");

    let mut treap = Treap { nodes: &mut nodes, root: NULL };
    (0..len).for_each(|node @ rank| treap.insert(node, rank));

    for node in iter {
        let rank = treap.remove(node);
        let new_rank = (treap.nodes[node].value + rank as i64).rem_euclid(len as i64 - 1) as usize;
        treap.insert(node, new_rank);
    }

    let zero_rank = treap.rank(zero);
    treap.nodes[treap.derank((zero_rank + 1000) % len)].value
        + treap.nodes[treap.derank((zero_rank + 2000) % len)].value
        + treap.nodes[treap.derank((zero_rank + 3000) % len)].value
}

pub fn part1(input: &Input) -> i64 {
    solve(input.len(), |n| input[n], 0..input.len())
}

pub fn part2(input: &Input) -> i64 {
    solve(
        input.len(),
        |n| input[n] * 811589153,
        (0..10).flat_map(|_| 0..input.len()),
    )
}

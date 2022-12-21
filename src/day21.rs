#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<Expr>;

#[derive(Clone, Copy)]
pub enum Expr {
    Const(i64),
    Op(usize, OpKind, usize),
}

#[derive(Clone, Copy)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

const ROOT: usize = 0;
const HUMN: usize = 1;

pub fn input_generator(input: &str) -> Input {
    let mut map = FxHashMap::from_iter([("root", ROOT), ("humn", HUMN)]);
    let mut output = vec![Expr::Const(0), Expr::Const(0)];
    let mut get_id = |name| {
        let next_id = map.len();
        *map.entry(name).or_insert(next_id)
    };

    input
        .lines()
        .map(|line| {
            let (key, rest) = line.split_once(": ").expect("Invalid input");
            let expr = rest.parse().map(Expr::Const).unwrap_or_else(|_| {
                let (l, rest) = rest.split_once(' ').expect("Invalid input");
                let (op, r) = rest.split_once(' ').expect("Invalid input");
                let op = match op {
                    "+" => OpKind::Add,
                    "-" => OpKind::Sub,
                    "*" => OpKind::Mul,
                    "/" => OpKind::Div,
                    _ => panic!("Invalid input"),
                };
                Expr::Op(get_id(l), op, get_id(r))
            });
            (get_id(key), expr)
        })
        .for_each(|(idx, expr)| {
            if output.len() <= idx {
                output.resize_with(idx + 1, || Expr::Const(0));
            }
            output[idx] = expr;
        });
    output
}

pub fn part1(input: &Input) -> i64 {
    fn eval(input: &Input, id: usize) -> i64 {
        match input[id] {
            Expr::Const(n) => n,
            Expr::Op(l, op, r) => {
                let l = eval(input, l);
                let r = eval(input, r);
                match op {
                    OpKind::Add => l + r,
                    OpKind::Sub => l - r,
                    OpKind::Mul => l * r,
                    OpKind::Div => l / r,
                }
            }
        }
    }
    eval(input, ROOT)
}

pub fn part2(input: &Input) -> i64 {
    use num::Integer;
    fn eval(input: &Input, id: usize) -> (i64, i64, i64) {
        if id == HUMN {
            return (1, 0, 1);
        }

        let (vl, vr, vd) = match input[id] {
            Expr::Const(n) => (0, n, 1),
            Expr::Op(l, op, r) => {
                let (ll, lr, ld) = eval(input, l);
                let (rl, rr, rd) = eval(input, r);
                match op {
                    OpKind::Add => (ll * rd + rl * ld, lr * rd + rr * ld, ld * rd),
                    OpKind::Sub => (ll * rd - rl * ld, lr * rd - rr * ld, ld * rd),
                    OpKind::Mul => (ll * rr + rl * lr, lr * rr, ld * rd),
                    OpKind::Div => (ll * rd, lr * rd, ld * rr),
                }
            }
        };
        let gcd = i64::gcd(&i64::gcd(&vd, &vl), &vr);
        (vl / gcd, vr / gcd, vd / gcd)
    }

    let Expr::Op(l, _, r) = input[ROOT] else { panic!() };
    let (ll, lr, ld) = eval(input, l);
    let (rl, rr, rd) = eval(input, r);

    let a = ll * rd - rl * ld;
    let b = rr * ld - lr * rd;
    b / a
}

#[allow(unused_imports)]
use super::prelude::*;
use std::ops::Add;

type Input<'input> = Dir<'input>;

pub fn input_generator(input: &str) -> Input<'_> {
    let mut commands = input
        .trim_start_matches("$ ")
        .split("\n$ ")
        .skip(1)
        .map(|command| command.split_once('\n').unwrap_or((command, "")));

    let mut root = Dir::default();
    exec_in_dir(&mut root, &mut commands);
    root
}

enum File<'a> {
    Dir(Dir<'a>),
    File(usize),
}

#[derive(Default)]
pub struct Dir<'a> {
    files: HashMap<&'a str, File<'a>>,
}

fn exec_in_dir<'a>(dir: &mut Dir<'a>, cmds: &mut impl Iterator<Item = (&'a str, &'a str)>) {
    while let Some((command, output)) = cmds.next() {
        match command {
            "ls" => dir.files.extend(output.lines().map(|line| {
                let (dirsize, name) = line.split_once(' ').expect("Invalid input");
                let file = match dirsize {
                    "dir" => File::Dir(Dir::default()),
                    _ => File::File(dirsize.parse().expect("Invalid input")),
                };
                (name, file)
            })),
            "cd .." => return,
            cd => {
                let Some(File::Dir(dir)) = dir.files.get_mut(&cd[3..]) else {
                    panic!()
                };
                exec_in_dir(dir, cmds);
            }
        }
    }
}

fn fold<M, F>(dir: &Dir, initial: usize, merge: M, finalize: F) -> usize
where
    M: Fn(usize, usize) -> usize,
    F: Fn(usize, usize) -> usize,
{
    fn helper<M, F>(dir: &Dir, initial: usize, merge: &M, finalize: &F) -> (usize, usize)
    where
        M: Fn(usize, usize) -> usize,
        F: Fn(usize, usize) -> usize,
    {
        let mut total = 0;
        let mut acc = initial;
        for file in dir.files.values() {
            match file {
                File::File(size) => total += size,
                File::Dir(dir) => {
                    let (size, new_acc) = helper(dir, initial, merge, finalize);
                    total += size;
                    acc = merge(acc, new_acc);
                }
            }
        }
        (total, finalize(acc, total))
    }

    helper(dir, initial, &merge, &finalize).1
}

pub fn part1(input: &Input) -> usize {
    fold(input, 0, usize::add, |acc, total| {
        acc + if total < 100_000 { total } else { 0 }
    })
}

pub fn part2(input: &Input) -> usize {
    let total = fold(input, 0, |_, _| 0, |_, total| total);
    let required = total - 40_000_000;

    fold(input, usize::MAX, usize::min, |acc, total| {
        match total >= required {
            true => min(total, acc),
            false => acc,
        }
    })
}

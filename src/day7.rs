#[allow(unused_imports)]
use super::prelude::*;
type Input<'input> = Dir<'input>;

pub fn input_generator(input: &str) -> Input<'_> {
    let commands = input
        .trim_start_matches("$ ")
        .split("\n$ ")
        .map(|command| {
            let (command, output) = command.split_once('\n').unwrap_or((command, ""));
            assert!(&command[..2] != "cd" || output.is_empty());
            (command, output.lines().collect())
        })
        .collect::<Vec<(_, Vec<_>)>>();

    let mut root = Dir::default();
    parse_dir(&mut root, &mut &commands[1..]);
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

fn parse_dir<'a, 'b>(curr_dir: &mut Dir<'a>, input: &mut &'b [(&'a str, Vec<&'a str>)]) {
    while let Some(((command, output), rest)) = input.split_first() {
        *input = rest;
        match *command {
            "ls" => curr_dir.files.extend(output.iter().map(|line| {
                let (dirsize, name) = line.split_once(' ').expect("Invalid input");
                let file = match dirsize {
                    "dir" => File::Dir(Dir::default()),
                    _ => File::File(dirsize.parse().expect("Invalid input")),
                };
                (name, file)
            })),
            "cd .." => return,
            cd => {
                let Some(File::Dir(dir)) = curr_dir.files.get_mut(&cd[3..]) else { panic!() };
                parse_dir(dir, input);
            }
        }
    }
}

fn generic_sum<M, F>(dir: &Dir, initial: usize, merge: M, finalize: F) -> usize
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
        for (_, file) in &dir.files {
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
    generic_sum(
        input,
        0,
        |acc, child| acc + child,
        |acc, total| acc + if total < 100_000 { total } else { 0 },
    )
}

pub fn part2(input: &Input) -> usize {
    let total = generic_sum(input, 0, |_, _| 0, |_, total| total);
    let required = total - 40_000_000;

    generic_sum(input, usize::MAX, min, |acc, total| {
        match total >= required {
            true => min(total, acc),
            false => acc,
        }
    })
}

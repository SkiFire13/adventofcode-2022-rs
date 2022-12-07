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

    let mut commands = &commands[..];
    let mut root = Dir::default();

    assert_eq!(commands[0].0, "cd /");
    while commands.len() != 0 {
        commands = parse_dir(&mut root, &commands[1..]);
    }
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

fn parse_dir<'a, 'b>(
    curr_dir: &mut Dir<'a>,
    mut input: &'b [(&'a str, Vec<&'a str>)],
) -> &'b [(&'a str, Vec<&'a str>)] {
    loop {
        let [(command, output), ..] = input else { return &[] };
        match &command[..2] {
            "ls" => {
                for line in output {
                    let (dirsize, name) = line.split_once(' ').expect("Invalid input");
                    let file = if dirsize == "dir" {
                        File::Dir(Dir::default())
                    } else {
                        File::File(dirsize.parse().expect("Invalid input"))
                    };
                    curr_dir.files.insert(name, file);
                }
            }
            "cd" => match &command[3..] {
                ".." | "/" => return input,
                dir => {
                    let Some(File::Dir(dir)) = curr_dir.files.get_mut(&dir) else { panic!() };
                    input = parse_dir(dir, &input[1..]);
                    let [(command, _), ..] = input else { return &[] };
                    match *command {
                        "cd .." => {}
                        "cd /" => return input,
                        _ => panic!("Invalid state"),
                    }
                }
            },
            _ => panic!("Invalid input"),
        }
        input = &input[1..]
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
        acc = finalize(acc, total);
        (total, acc)
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
        if total >= required {
            min(total, acc)
        } else {
            acc
        }
    })
}

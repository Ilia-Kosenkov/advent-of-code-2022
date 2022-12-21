use std::{
    collections::HashMap,
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

fn unit<T>(_: T) {}

#[derive(Debug, Clone)]
struct FileInDir<'a> {
    file: File,
    dir_path: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    full_path: String,
    size: usize,
}

#[derive(Debug)]
enum Command {
    Cd { path: String },
}

#[derive(Debug)]
enum Item {
    File { name: String, size: usize },
}

#[derive(Debug)]
enum Input {
    Command(Command),
    Item(Item),
    Discard,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd") {
            Ok(Input::Command(Command::Cd {
                path: s[4..].trim().to_string(),
            }))
        } else if s.starts_with("dir") || s.starts_with("$ ls") {
            Ok(Input::Discard)
        } else {
            let mut split = s.trim().split_whitespace();
            let size = split.next().and_then(|s| s.parse::<usize>().ok());
            let name = split.next().map(|s| s.to_string());

            match (size, name) {
                (Some(size), Some(name)) => Ok(Input::Item(Item::File {
                    name: name,
                    size: size,
                })),
                _ => Err(()),
            }
        }
    }
}

fn main() {
    const THRESHOLD: usize = 100_000;
    const MAX_CAPACITY: usize = 70_000_000;
    const REQUIRED_CAPACITY: usize = 30_000_000;

    let reader = io::BufReader::new(io::stdin());
    let input = reader
        .lines()
        .map(|l| l.map_err(unit).and_then(|s| s.parse::<Input>()));

    let result = build_fs_tree(input);

    if let Ok(result) = result {
        let files = result
            .iter()
            .flat_map(|f| {
                get_folders(f)
                    .into_iter()
                    .map(|d| FileInDir {
                        file: f.clone(),
                        dir_path: d,
                    })
                    .collect_vec()
            })
            .collect::<Vec<_>>();

        let mut dir_sizes = HashMap::<&str, usize>::new();

        for file_in_dir in files {
            let entry = dir_sizes.entry(file_in_dir.dir_path);
            *entry.or_default() += file_in_dir.file.size;
        }

        let small_dirs_size = total_size(dir_sizes.iter(), |(_, size)| **size <= THRESHOLD);
        let total_size = dir_sizes[""];
        let available = MAX_CAPACITY - total_size;
        let deficit = REQUIRED_CAPACITY - available;

        let smallest_dir = dir_sizes
            .iter()
            .filter_map(|(_, size)| if *size >= deficit { Some(*size) } else { None })
            .sorted()
            .next();

        println!("{}", small_dirs_size);
        println!("{}", smallest_dir.unwrap_or(<usize>::MAX));
    }
}

fn total_size<'a, 'b, T, F>(dirs: T, predicate: F) -> usize
where
    'b: 'a,
    T: Iterator<Item = (&'a &'b str, &'a usize)>,
    F: FnMut(&(&&str, &usize)) -> bool,
{
    dirs.filter(predicate).map(|x| x.1).sum::<usize>()
}

fn build_fs_tree<T: Iterator<Item = Result<Input, ()>>>(input: T) -> Result<Vec<File>, ()> {
    use Command::*;

    let mut current_path = "".to_string();
    let mut files = Vec::new();

    for item in input {
        if let Ok(item) = item {
            if let Input::Command(Cd { path }) = item {
                change_path(&mut current_path, &path);
            } else if let Input::Item(Item::File { name, size }) = item {
                let mut file_path = current_path.clone();
                change_path(&mut file_path, &name);
                let file = File {
                    full_path: file_path,
                    size: size,
                };
                files.push(file);
            }
        } else {
            return Err(());
        }
    }

    Ok(files)
}

fn change_path(current_path: &mut String, path: &str) {
    if current_path.is_empty() {
        if path != "/" {
            current_path.push('/')
        }
        current_path.push_str(path);
    } else if path == ".." {
        while let Some(char) = current_path.pop() {
            if char == '/' {
                break;
            }
        }
    } else {
        if !current_path.ends_with('/') {
            current_path.push_str("/");
        }
        current_path.push_str(path);
    }
}

fn get_folders(file: &File) -> Vec<&str> {
    file.full_path
        .char_indices()
        .filter_map(|(i, c)| {
            if c == '/' {
                Some(&file.full_path[..i])
            } else {
                None
            }
        })
        .collect()
}

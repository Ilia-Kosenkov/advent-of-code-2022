use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    ops::Add,
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
    Ls,
    Cd { path: String },
}

#[derive(Debug)]
enum Item {
    File { name: String, size: usize },
    Dir { name: String },
}

#[derive(Debug)]
enum Input {
    Command(Command),
    Item(Item),
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ls") {
            Ok(Input::Command(Command::Ls))
        } else if s.starts_with("$ cd") {
            Ok(Input::Command(Command::Cd {
                path: s[4..].trim().to_string(),
            }))
        } else if s.starts_with("dir") {
            Ok(Input::Item(Item::Dir {
                name: s[3..].trim().to_string(),
            }))
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
    let reader = io::BufReader::new(io::stdin());
    let input = reader
        .lines()
        .map(|l| l.map_err(unit).and_then(|s| s.parse::<Input>()));

    let result = build_fs_tree(input);

    let test = result.unwrap();
    dbg! { &test };

    let files = test
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

    let small_dirs = dir_sizes
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .collect_vec();
    let sum = small_dirs.iter().map(|(_, size)| size).sum::<usize>();

    dbg! { &small_dirs };
    println!("{}", sum);
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

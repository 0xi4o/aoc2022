#![feature(iter_intersperse)]
use std::collections::BTreeMap;

#[derive(Debug)]
#[derive(Clone)]
struct File {
    name: String,
    size: u64
}

#[derive(Debug)]
struct Directory {
    path: String,
    files: Vec<File>
}

pub fn part1(input: &str) -> u64 {
    let mut pwd = Directory { path: "/".to_string(), files: Vec::new() };
    let mut directories: Vec<Directory> = Vec::new();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let _ = lines
        .iter()
        .map(|line| {
            let command_parts = line.split_whitespace().collect::<Vec<&str>>();
            if command_parts[0] == "$" && command_parts[1] == "cd" {
                match command_parts[2] {
                    "/" => {
                        let root_dir = Directory { path: "/".to_string(), files: Vec::new() };
                        directories.push(root_dir);
                    },
                    ".." => {
                        let mut path: String = "/".to_string();
                        let mut path_parts = pwd.path.split("/").collect::<Vec<&str>>();
                        if path_parts.len() > 2 {
                            path_parts.pop();
                            path = path_parts.join("/");
                        } else if path_parts.len() == 2 {
                            path = "/".to_string();
                        }
                        let curr = directories.iter().find(|dir| dir.path == path);
                        pwd = match curr {
                            Some(curr) => {
                                Directory {
                                    path: curr.path.to_string(),
                                    files: curr.files.iter().map(|file| File::clone(file)).collect()
                                }
                            },
                            None => {
                                Directory { path: path.to_string(), files: Vec::new() }
                            }
                        }
                    },
                    _ => {
                        let path: String;
                        if pwd.path != "/" {
                            path = pwd.path.to_string() + &"/".to_string() + command_parts[2];
                        } else {
                            path = pwd.path.to_string() + command_parts[2];
                        }
                        let curr = directories.iter().find(|dir| dir.path == path);
                        pwd = match curr {
                            Some(curr) => {
                                Directory {
                                    path: path.to_string(),
                                    files: curr.files.iter().map(|file| File::clone(file)).collect()
                                }
                            },
                            None => {
                                Directory { path: path.to_string(), files: Vec::new() }
                            }
                        }
                    }
                }
            } else if command_parts[0] == "$" && command_parts[1] == "ls" {

            } else if command_parts[0] == "dir" {
                let path: String;
                if pwd.path != "/" {
                    path = pwd.path.to_string() + &"/".to_string() + command_parts[1];
                } else {
                    path = pwd.path.to_string() + command_parts[1];
                }
                let dir = Directory { path: path.to_string(), files: Vec::new() };
                directories.push(dir);
            } else {
                for dir in directories.iter_mut() {
                    if dir.path == pwd.path {
                        let file = File { name: command_parts[1].to_string(), size: command_parts[0].parse::<u64>().unwrap() };
                        dir.files.push(file);
                    }
                }
            }

            *line
        })
        .collect::<Vec<&str>>();

    let mut sizes: BTreeMap<String, u64> = BTreeMap::new();
    for dir in directories.iter() {
        let dirs = dir.path.split("/").collect::<Vec<&str>>();
        let size = dir.files
            .iter()
            .map(|file| file.size)
            .sum::<u64>();

        for i in 0..dirs.len() {
            sizes.entry(
                (&dirs[0..=i])
                    .iter()
                    .cloned()
                    .intersperse("/")
                    .collect::<String>()
            )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let result = sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u64>();
    result
}

pub fn part2(input: &str) -> u64 {
    let mut pwd = Directory { path: "/".to_string(), files: Vec::new() };
    let mut directories: Vec<Directory> = Vec::new();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let _ = lines
        .iter()
        .map(|line| {
            let command_parts = line.split_whitespace().collect::<Vec<&str>>();
            if command_parts[0] == "$" && command_parts[1] == "cd" {
                match command_parts[2] {
                    "/" => {
                        // let root_dir = Directory { path: "/".to_string(), files: Vec::new() };
                        // directories.push(root_dir);
                    },
                    ".." => {
                        let mut path: String = "/".to_string();
                        let mut path_parts = pwd.path.split("/").collect::<Vec<&str>>();
                        if path_parts.len() > 2 {
                            path_parts.pop();
                            path = path_parts.join("/");
                        } else if path_parts.len() == 2 {
                            path = "/".to_string();
                        }
                        let curr = directories.iter().find(|dir| dir.path == path);
                        pwd = match curr {
                            Some(curr) => {
                                Directory {
                                    path: curr.path.to_string(),
                                    files: curr.files.iter().map(|file| File::clone(file)).collect()
                                }
                            },
                            None => {
                                Directory { path: path.to_string(), files: Vec::new() }
                            }
                        }
                    },
                    _ => {
                        let path: String;
                        if pwd.path != "/" {
                            path = pwd.path.to_string() + &"/".to_string() + command_parts[2];
                        } else {
                            path = pwd.path.to_string() + command_parts[2];
                        }
                        let curr = directories.iter().find(|dir| dir.path == path);
                        pwd = match curr {
                            Some(curr) => {
                                Directory {
                                    path: path.to_string(),
                                    files: curr.files.iter().map(|file| File::clone(file)).collect()
                                }
                            },
                            None => {
                                Directory { path: path.to_string(), files: Vec::new() }
                            }
                        }
                    }
                }
            } else if command_parts[0] == "$" && command_parts[1] == "ls" {

            } else if command_parts[0] == "dir" {
                let path: String;
                if pwd.path != "/" {
                    path = pwd.path.to_string() + &"/".to_string() + command_parts[1];
                } else {
                    path = pwd.path.to_string() + command_parts[1];
                }
                let dir = Directory { path: path.to_string(), files: Vec::new() };
                directories.push(dir);
            } else {
                for dir in directories.iter_mut() {
                    if dir.path == pwd.path {
                        let file = File { name: command_parts[1].to_string(), size: command_parts[0].parse::<u64>().unwrap() };
                        dir.files.push(file);
                    }
                }
            }

            *line
        })
        .collect::<Vec<&str>>();

    let mut sizes: BTreeMap<String, u64> = BTreeMap::new();
    for dir in directories.iter() {
        let dirs = dir.path.split("/").collect::<Vec<&str>>();
        let size = dir.files
            .iter()
            .map(|file| file.size)
            .sum::<u64>();

        for i in 0..dirs.len() {
            sizes.entry(
                (&dirs[0..=i])
                    .iter()
                    .cloned()
                    .intersperse("/")
                    .collect::<String>()
            )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let total_space: u64 = 70000000;
    let required_space:u64 = 30000000;
    let used_space = sizes.get("").unwrap();
    dbg!(&used_space);
    let free_space = total_space - used_space;
    dbg!(&free_space);
    let need_to_free_atleast = required_space - free_space;
    dbg!(&need_to_free_atleast);


    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, size)| size > &&need_to_free_atleast)
        .map(|(_, size)| size)
        .collect::<Vec<&u64>>();

    valid_dirs.sort();
    dbg!(&valid_dirs);
    valid_dirs.iter().next().unwrap().to_string().parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 24933642);
    }
}

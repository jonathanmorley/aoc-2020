use std::collections::HashMap;

use itertools::Itertools;

type FilePath<'a> = Vec<&'a str>;

#[derive(Debug)]
pub struct File {
    size: u128,
}

#[derive(Debug)]
pub struct Directory;

#[derive(Debug)]
pub struct FileSystem<'a> {
    files: HashMap<FilePath<'a>, File>,
    directories: HashMap<FilePath<'a>, Directory>,
}

impl<'a> FileSystem<'a> {
    fn size(&self, path: &FilePath<'a>) -> u128 {
        self.files
            .iter()
            .filter(|(p, _)| p.starts_with(path))
            .map(|(_, f)| f.size)
            .sum()
    }
}

pub fn parse(s: &str) -> FileSystem {
    let mut fs = FileSystem {
        files: HashMap::new(),
        directories: HashMap::new(),
    };

    let mut cwd = FilePath::new();

    for line in s.lines() {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", "/"] => {
                cwd = Vec::new();
                fs.directories.insert(cwd.clone(), Directory);
            }
            ["$", "cd", ".."] => {
                cwd.pop();
                fs.directories.insert(cwd.clone(), Directory);
            }
            ["$", "cd", name] => {
                cwd.push(*name);
                fs.directories.insert(cwd.clone(), Directory);
            }
            ["$", "ls"] => {
                println!("listing /{}/", cwd.join("/"));
            }
            ["dir", name] => {
                let mut path = cwd.clone();
                path.push(*name);

                fs.directories.insert(path, Directory);
            }
            [size, name] => {
                let mut path = cwd.clone();
                path.push(*name);

                fs.files.insert(
                    path,
                    File {
                        size: size.parse().unwrap(),
                    },
                );
            }
            _ => unreachable!(),
        }
    }

    fs
}

pub fn part1(fs: &FileSystem) -> u128 {
    fs.directories
        .keys()
        .map(|d| fs.size(d))
        .filter(|size| *size <= 100_000)
        .sum()
}

pub fn part2(fs: &FileSystem) -> u128 {
    let unused = 70_000_000 - fs.size(&vec![]);
    let required = 30_000_000 - unused;

    fs.directories
        .keys()
        .map(|d| fs.size(d))
        .filter(|size| *size > required)
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$ cd /
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
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(SAMPLE)), 24933642);
    }
}

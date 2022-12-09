use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let input_path = env::args().skip(1).next().expect("give input file");
    let contents = fs::read_to_string(input_path).expect("cannot read file");

    let mut mock_fs = MockFileSystem::new();
    for line in contents.lines() {
        mock_fs.parse_line(line);
    }
    println!("fs = {:?}", mock_fs);
    println!("total size to clean: {}", sum_small_dir_size(&mock_fs, 100000));
    println!("smallest size to clean: {:?}", smallest_dir_size_to_delete(&mock_fs, 30000000));
}

#[derive(Debug)]
struct Dir {
    subdirs: Vec<String>,
    total_size: usize,
}

#[derive(Debug)]
struct MockFileSystem {
    paths: HashMap<String, Dir>,
    cwd: String,
}

impl Dir {
    fn new() -> Dir {
        Dir { subdirs: vec![], total_size: 0 }
    }
}

impl MockFileSystem {
    fn new() -> MockFileSystem {
        MockFileSystem {
            paths: HashMap::from([
                ("/".to_string(), Dir::new())
            ]),
            cwd: "/".to_string()
        }
    }

    fn parent(path: &str) -> String {
        match path.rsplit_once("/") {
            Some(("/", _)) | Some(("", _)) => String::from("/"),
            Some((a, _)) => a.to_string(),
            None => String::from("/"),
        }
    }

    fn subpath(&self, stem: &str) -> String {
        if self.cwd.ends_with("/") {
            String::from(&self.cwd) + stem
        } else {
            String::from(&self.cwd) + &String::from("/") + stem
        }
    }

    fn parse_line(&mut self, line: &str) {
        let mut tokens = line.split(" ").peekable();
        match tokens.peek() {
            Some(&"$") => self.run_command(&tokens.skip(1).collect()),
            Some(&"dir") => self.record_dir(&tokens.collect()),
            Some(_) => self.record_file(&tokens.collect()),
            None => ()
        }
    }

    fn run_command(&mut self, tokens: &Vec<&str>) {
        match tokens[0] {
            "ls" => (),
            "cd" => {
                match tokens[1] {
                    ".." => self.cwd = MockFileSystem::parent(&self.cwd),
                    d => {
                        self.cwd = if d.starts_with("/") {
                            d.to_string()
                        } else {
                            self.subpath(d)
                        };
                    }
                }
            },
            &_ => panic!("unsupported command")
        }
    }

    fn record_dir(&mut self, tokens: &Vec<&str>) {
        let full_path = self.subpath(tokens[1]);
        if let Some(_) = self.paths.get(&full_path) {
            panic!("Directory {} already seen", full_path);
        }
        self.paths.insert(full_path.clone(), Dir::new());

        let parent = self.paths.get_mut(&self.cwd).expect("no parent");
        parent.subdirs.push(full_path);
    }

    fn record_file(&mut self, tokens: &Vec<&str>) {
        let mut path = String::from(&self.cwd);
        let size: usize = tokens[0].parse().expect("not a number");
        while path != "/" {
            let mut parent = self.paths.get_mut(&path).expect("no parent");
            parent.total_size += size;
            path = MockFileSystem::parent(&path);
        }
        self.paths.get_mut("/").expect("no root").total_size += size;

    }

    fn total_size(&self) -> usize {
        self.paths.get("/").expect("no root").total_size
    }

    fn free_space(&self) -> usize {
        // disk space is defined to be 70000000
        let total_disk_space = 70000000;
        total_disk_space - self.total_size()
    }
}

fn sum_small_dir_size(fs: &MockFileSystem, max_size: usize) -> usize {
    fs.paths.iter().map(|(_, v)| v.total_size).filter(|s| s <= &max_size).sum()
}

fn smallest_dir_size_to_delete(fs: &MockFileSystem, target_size: usize) -> Option<usize> {
    let min_size = target_size - fs.free_space();
    fs.paths.iter().map(|(_, v)| v.total_size).filter(|s| s >= &min_size).min()
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "$ cd /
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
7214296 k"
    }

    #[test]
    fn test_parser() {
        let mut mock_fs = MockFileSystem::new();
        for line in sample().lines() {
            mock_fs.parse_line(line);
        }

        assert_eq!(mock_fs.paths.get("/a/e").unwrap().total_size, 584);
        assert_eq!(mock_fs.paths.get("/a").unwrap().total_size, 94853);
        assert_eq!(mock_fs.paths.get("/d").unwrap().total_size, 24933642);
        assert_eq!(mock_fs.total_size(), 48381165);
        assert_eq!(mock_fs.free_space(), 21618835);
        assert_eq!(sum_small_dir_size(&mock_fs, 100000), 95437);
        assert_eq!(smallest_dir_size_to_delete(&mock_fs, 30000000), Some(24933642));
    }

    #[test]
    fn test_rsplit() {
        assert_eq!("/foo/bar".rsplit_once("/"), Some(("/foo", "bar")));
        assert_eq!("/foo".rsplit_once("/"), Some(("", "foo")));
        assert_eq!("/".rsplit_once("/"), Some(("", "")));
    }

    #[test]
    fn test_mock_fs_parent() {
        assert_eq!(MockFileSystem::parent("/"), "/");
        assert_eq!(MockFileSystem::parent("/foo"), "/");
        assert_eq!(MockFileSystem::parent("/foo/bar"), "/foo");
    }
}

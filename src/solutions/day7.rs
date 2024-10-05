use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::AoCDay;

pub struct Code;

#[derive(Debug)]
struct File {
    name: String,
    size: i64,
}

#[derive(Debug)]
struct Directory {
    files: Vec<File>,
    dirs: Vec<String>,
    parent: Option<String>,
    size: i64,
}

impl Directory {
    pub fn new(parent: Option<String>) -> Self {
        Self {
            files: vec![],
            dirs: vec![],
            parent,
            size: 0,
        }
    }
}

enum Command {
    Cd(String),
    Ls,
}

fn parse_command(input: &str) -> Option<Command> {
    if input.starts_with("$ cd") {
        let re = Regex::new(r"^\$ cd (?P<target>.+)$").unwrap();
        Some(Command::Cd(
            re.captures(input)
                .unwrap()
                .name("target")
                .unwrap()
                .as_str()
                .to_owned(),
        ))
    } else if input.starts_with("$ ls") {
        Some(Command::Ls)
    } else {
        None
    }
}

enum Thingy {
    Dir(String),
    File(File),
}

fn parse_output(input: &str) -> Thingy {
    if input.starts_with("dir") {
        let re = Regex::new(r"^dir (?P<dir>.+)$").unwrap();
        Thingy::Dir(
            re.captures(input)
                .unwrap()
                .name("dir")
                .unwrap()
                .as_str()
                .to_owned(),
        )
    } else {
        let re = Regex::new(r"^(?P<size>\d+) (?P<name>.+)$").unwrap();
        let size = re
            .captures(input)
            .unwrap()
            .name("size")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let name = re
            .captures(input)
            .unwrap()
            .name("name")
            .unwrap()
            .as_str()
            .to_owned();
        Thingy::File(File { name, size })
    }
}

fn make_filesystem_lol(input: &str) -> HashMap<String, Rc<RefCell<Directory>>>  {
    let mut all_dirs: HashMap<String, Rc<RefCell<Directory>>> = HashMap::new();
        let mut current = Rc::new(RefCell::new(Directory::new(None)));

        all_dirs.insert("/".to_owned(), current.clone());
        let mut current_path = vec![];

        for line in input.lines().skip(1) {
            let command = parse_command(line);
            if let Some(command) = command {
                match command {
                    Command::Cd(target) => {
                        if target == ".." {
                            current_path.pop();
                        } else {
                            current_path.push(target);
                        }
                        let path_string = current_path.join("/");
                        current = all_dirs.get(&format!("/{path_string}")).unwrap().clone();
                    }
                    Command::Ls => {}
                }
            } else {
                let output = parse_output(line);
                match output {
                    Thingy::Dir(dir) => {
                        let (new_dir_path, current_path) = if current_path.is_empty() {
                            (format!("/{}", dir), "/".to_owned())
                        } else {
                            let joind = current_path.iter().join("/");
                            (format!("/{joind}/{dir}"), format!("/{joind}"))
                        };
                        let mut c = current.borrow_mut();
                        c.dirs.push(dir.clone());
                        let d = Rc::new(RefCell::new(Directory::new(Some(current_path))));
                        all_dirs.insert(new_dir_path, d);
                    }
                    Thingy::File(f) => {
                        let size = {
                            let mut c = current.borrow_mut();
                            let size = f.size;
                            c.size += size;
                            c.files.push(f);
                            size
                        };

                        let mut parent = current.borrow().parent.clone();
                        while let Some(parent_path) = parent {
                            let parent_dir = all_dirs.get(&parent_path);
                            if let Some(p) = parent_dir {
                                let mut c = p.borrow_mut();
                                c.size += size;
                               
                                parent = c.parent.as_ref().cloned()
                            } else {
                                parent = None;
                            }
                        }
                    }
                }
            }
        }
        all_dirs
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let file_system = make_filesystem_lol(input);
        let total = file_system
            .iter()
            .filter(|(_k, v)| v.borrow().size <= 100000)
            .map(|(k,v)| {v.borrow().size})
            .sum::<i64>();
        format!("{total}") // 1778099
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let file_system = make_filesystem_lol(input);
        let available = 70000000;
        let needed = 30000000;
    
        let total_size = file_system.get("/").unwrap().borrow().size;
        let unused = available - total_size;
        let target = needed - unused;

        let smallest_dir = file_system
            .iter()
            .filter(|(_k, v)| v.borrow().size >= target)
            .map(|(k,v)| {v.borrow().size})
            .min().unwrap();

        format!("{smallest_dir}")
    }
}

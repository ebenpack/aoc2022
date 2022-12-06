#![allow(unused_variables)]

use regex::{Captures, Regex};
use std::{collections::VecDeque, str::FromStr};

use crate::AoCDay;

pub struct Code;

fn parse_capture<T>(capture: &Captures, name: &str) -> T
where
    T: FromStr,
{
    let result = capture.name(name).unwrap().as_str().parse::<T>();
    match result {
        Ok(val) => val,
        _ => panic!(),
    }
}

fn solve(
    input: &str,
    mutate_stack: fn(&mut Vec<VecDeque<char>>, i32, usize, usize) -> (),
) -> String {
    // Part 1
    let mut stacks = vec![VecDeque::new(); 9];

    let re = Regex::new(r"^move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();

    //  TODO: This is kinda hinky
    let mut part_one = true;
    for line in input.lines() {
        if part_one {
            if line.is_empty() {
                part_one = false;
                continue;
            }
            for (idx, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    let index = idx / 4;
                    stacks[index].push_back(c);
                }
            }
        } else {
            // Part 2
            let capture = re.captures(line).unwrap();

            let num = parse_capture::<i32>(&capture, "num");
            let from = parse_capture::<usize>(&capture, "from");
            let to = parse_capture::<usize>(&capture, "to");
            mutate_stack(&mut stacks, num, from - 1, to - 1);
        }
    }
    stacks
        .iter()
        .map(|s| *s.front().unwrap())
        .collect::<String>()
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        solve(input, |stacks, num, from, to| {
            let mut num = num;
            while num > 0 {
                let val = stacks[from].pop_front().unwrap();
                stacks[to].push_front(val);
                num -= 1;
            }
        })
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        solve(input, |stacks, num, from, to| {
            let mut v = VecDeque::new();
            let mut num = num;
            while num > 0 {
                let val = stacks[from].pop_front().unwrap();
                v.push_front(val);
                num -= 1;
            }
            for val in v.iter() {
                stacks[to].push_front(*val);
            }
        })
    }
}

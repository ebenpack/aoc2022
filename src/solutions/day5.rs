#![allow(unused_variables)]

use regex::Regex;
use std::collections::VecDeque;

use crate::AoCDay;

pub struct Code;

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

            let num = capture
                .name("num")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let from = capture
                .name("from")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let to = capture
                .name("to")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

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

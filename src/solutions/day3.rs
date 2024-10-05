use itertools::Itertools;
use std::collections::HashSet;

use crate::AoCDay;

pub struct Code;

fn char_to_u32(c: &char) -> u32 {
    if c.is_lowercase() {
        (*c as u32) - 96
    } else {
        (*c as u32) - 38
    }
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut total = 0;
        'outer: for line in input.lines() {
            let (left, right) = line.split_at(line.len() / 2);
            let first_set: HashSet<char> = HashSet::from_iter(left.chars());
            for c in right.chars() {
                if first_set.contains(&c) {
                    total += char_to_u32(&c);
                    continue 'outer;
                }
            }
        }
        format!("{total}") // 7691
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut total = 0;
        for group in &input.lines().chunks(3) {
            let all: HashSet<char> = group.fold(HashSet::new(), |acc, x| {
                // Bleh :(
                if acc.is_empty() {
                    return HashSet::from_iter(x.chars());
                }
                let mut res = HashSet::new();
                for x in acc.intersection(&HashSet::from_iter(x.chars())) {
                    res.insert(x.to_owned());
                }
                res
            });
            total += all.iter().map(char_to_u32).sum::<u32>();
        }
        format!("{total}") // 2508
    }
}

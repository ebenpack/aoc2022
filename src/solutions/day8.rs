#![allow(unused_variables)]

use crate::AoCDay;

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut thingus = vec![];
        for c in input.lines().chars() {
            thingus.push(c.parse::<i32>().unwrap())
        }
        todo!()
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        todo!()
    }
}

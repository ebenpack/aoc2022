#![allow(unused_variables)]

use crate::AoCDay;

pub struct Code;

#[derive(PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn val(&self) -> i32 {
        match self {
            Shape::Rock => 0,
            Shape::Paper => 1,
            Shape::Scissors => 2,
        }
    }
    fn score(&self) -> i32 {
        self.val() + 1
    }
    fn from_i32(i: i32) -> Self {
        match i {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn parse_play(c: char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!(),
    }
}

fn parse_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn score_part_one(other: &Shape, me: &Shape) -> i32 {
    let game_score = if (other.val() + 1) % 3 == me.val() {
        6
    } else if other.val() == me.val() {
        3
    } else {
        0
    };
    game_score + me.score()
}

fn score_part_two(other: &Shape, me: &Outcome) -> i32 {
    let my_play = Shape::from_i32(match me {
        Outcome::Win => (other.val() + 1) % 3,
        Outcome::Lose => (other.val() + 2) % 3,
        Outcome::Draw => other.val(),
    });

    score_part_one(other, &my_play)
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let total = input
            .lines()
            .map(|line| {
                let mut b = line.chars();
                let other = parse_play(b.next().unwrap());
                let me = parse_play(b.nth(1).unwrap());
                score_part_one(&other, &me)
            })
            .sum::<i32>();
        format!("{total}")
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let total = input
            .lines()
            .map(|line| {
                let mut b = line.chars();
                let other = parse_play(b.next().unwrap());
                let me = parse_outcome(b.nth(1).unwrap());
                score_part_two(&other, &me)
            })
            .sum::<i32>();
        format!("{total}")
    }
}

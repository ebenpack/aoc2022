use crate::AoCDay;

pub struct Code;

fn elf_list(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.split("\n\n").map(|elf| {
        elf.trim()
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum()
    })
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let list = elf_list(input);
        let max = list.max().unwrap();
        format!("{max}")
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut list = elf_list(input).collect::<Vec<_>>();
        list.sort();
        let max_three_sum = list.iter().rev().take(3).sum::<i32>();
        format!("{max_three_sum}")
    }
}

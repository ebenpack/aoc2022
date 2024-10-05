use hashbrown::HashSet;

use crate::AoCDay;

pub struct Code;

fn start_of_packet(input: &str, n: usize) -> bool {
    let slice = &input[n..n + 4];
    slice.chars().collect::<HashSet<_>>().len() == 4
}

fn start_of_message(input: &str, n: usize) -> bool {
    let slice = &input[n..n + 14];
    slice.chars().collect::<HashSet<_>>().len() == 14
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut result = 0;
        for x in 0..input.len() - 4 {
            if start_of_packet(input, x) {
                result = x + 4;
                break;
            }
        }
        format!("{result}") // 1271 low
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut result = 0;
        for x in 0..input.len() - 4 {
            if start_of_message(input, x) {
                result = x + 14;
                break;
            }
        }
        format!("{result}") // 1271 low
    }
}

// abcdef
// 0123

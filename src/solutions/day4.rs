use crate::AoCDay;

pub struct Code;

type Range = (i32, i32);

fn split(s: &str, c: char) -> (&str, &str) {
    let s = s.split(c).collect::<Vec<_>>();
    (s.first().unwrap(), s.get(1).unwrap())
}

fn get_range(s: &str) -> Range {
    let (l, r) = split(s, '-');
    (l.parse().unwrap(), r.parse().unwrap())
}

fn range_contains(l: Range, r: Range) -> bool {
    r.0 >= l.0 && r.0 <= l.1 && r.1 >= l.0 && r.1 <= l.1
}

fn range_overlaps(l: Range, r: Range) -> bool {
    (r.0 >= l.0 && r.0 <= l.1) || (r.1 >= l.0 && r.1 <= l.1)
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let count = input
            .lines()
            .map(|line| {
                let s = line.split(',').collect::<Vec<_>>();
                let (l, r) = (s.first().unwrap(), s.get(1).unwrap());
                let (left, right) = (get_range(l), get_range(r));
                i32::from(range_contains(left, right) || range_contains(right, left))
            })
            .sum::<i32>();
        format!("{count}")
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let count = input
            .lines()
            .map(|line| {
                let s = line.split(',').collect::<Vec<_>>();
                let (l, r) = (s.first().unwrap(), s.get(1).unwrap());
                let (left, right) = (get_range(l), get_range(r));
                i32::from(range_overlaps(left, right) || range_overlaps(right, left))
            })
            .sum::<i32>();
        format!("{count}")
    }
}

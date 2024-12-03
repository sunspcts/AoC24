use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt does not exist.");
    let reports: Vec<Vec<i64>> = input.lines().map(parse_line).collect();
    println!("{}", part_1(&reports));
    println!("{}", part_2(&reports));
}

fn part_1(reports: &[Vec<i64>]) -> usize {
    reports.iter().filter(|x| safe(x)).count()
}

fn part_2(reports: &[Vec<i64>]) -> usize {
    reports.iter().filter(|x| problem_dampener(x)).count()
}

fn problem_dampener(report: &[i64]) -> bool {
    let mut v = vec![];
    for i in 0..report.len() {
        v.push([&report[..i], &report[(i + 1)..]].concat());
    } 
    v.iter().any(|x| safe(x))
}

fn safe(report: &[i64]) -> bool {
    match is_monotonic(report) {
        MonotonicBehaviour::Inc | MonotonicBehaviour::Dec => {
            report.iter().tuple_windows().all(|(x, y)| (x - y).abs() <= 3 &&  (x - y).abs() >= 1)
        },
        MonotonicBehaviour::Neither => false,
    }
}

fn is_monotonic(arr: &[i64]) -> MonotonicBehaviour {
    let inc = arr.iter().tuple_windows().all(|(x, y)| x <= y);
    let dec = arr.iter().tuple_windows().all(|(x, y)| x >= y);
    match (inc, dec) {
        (true, false) => MonotonicBehaviour::Inc,
        (false, true) => MonotonicBehaviour::Dec,
        _ => MonotonicBehaviour::Neither
    }
}

enum MonotonicBehaviour {
    Inc, Dec, Neither
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
    .map(|n| n.parse::<i64>().expect("Not a Number!"))
    .collect()
}
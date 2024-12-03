use std::collections::HashMap;
use std::fs;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt does not exist.");
    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = input.lines().map(parse_line).unzip();
    list_a.sort_unstable(); list_b.sort_unstable();
    println!("{}", part_1(&list_a, &list_b));
    println!("{}", part_2(&list_a, &list_b));
}

fn part_1(list_a: &[i32], list_b: &[i32]) -> i32 {
    zip(list_a, list_b).map(|(x, y)| (x - y).abs()).sum()
}

fn part_2(list_a: &[i32], list_b: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in list_b {
        *map.entry(*i).or_default() += 1;
    }
    list_a.iter().map(|x| 
        map.get(x).map_or(0, |n| n * x)).sum()
}


fn parse_line(line: &str) -> (i32, i32) {
    let split: Vec<i32> = line.split_whitespace()
    .map(|n| n.parse::<i32>().expect("Not a Number!"))
    .collect();
    assert!(split.len() == 2);
    (split[0], split[1])
}
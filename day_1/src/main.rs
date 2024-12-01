use std::collections::HashMap;
use std::fs;
use std::iter::zip;
fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt does not exist.");
    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = input.split("\n").map(parse_line).unzip();
    list_a.sort(); list_b.sort();
    println!("{}", part_1(&list_a, &list_b));
    println!("{}", part_2(&list_a, &list_b));
}

fn part_1(list_a: &[i32], list_b: &[i32]) -> i32 {
    zip(list_a, list_b).fold(0, |acc, (x, y)| acc + (x - y).abs())
}

fn part_2(list_a: &[i32], list_b: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in list_b {
        map.entry(*i).and_modify(|x| *x += 1).or_insert(1);
    }

    list_a.iter().map(|x| 
        map.get(x).map_or(0, |n| n * x)).sum()
}


fn parse_line(line: &str) -> (i32, i32) {
    let split: Vec<i32> = line.split_whitespace()
    .map(|n| n.parse::<i32>().expect("Not a Number!"))
    .collect();
    match split.len() {
        0 => (0, 0),
        _ => (split[0], split[1])
    }
}
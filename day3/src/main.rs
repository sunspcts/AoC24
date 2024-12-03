// this SUCKS and is HACKY but it WORKS and that's COOL
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![deny(clippy::all)]
use regex::Regex;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt does not exist.");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut pairs: Vec<(u64, u64)>  = vec![];
    for (_, [x, y]) in re.captures_iter(&input).map(|c| c.extract()) {
        pairs.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    println!("{}", pairs.into_iter().map(|(x, y)| x*y).sum::<u64>());
    let split:Vec<&str> = input.split("don't()").collect();
    let mut splitt = vec![split[0].to_owned()];
    for s in split {
        let v:Vec<&str> = s.split("do()").collect();
            splitt.push(v[1..].concat());
        
    }
    let splitt: String = splitt.join("");
    pairs = vec![];
    for (_, [x, y]) in re.captures_iter(&splitt).map(|c| c.extract()) {
        pairs.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    println!("{}", pairs.into_iter().map(|(x, y)| x*y).sum::<u64>());

}

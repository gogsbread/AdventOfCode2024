use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter;

use aoc_2024::*;

fn p1(input: &Vec<String>) -> i32 {
    let (mut first, mut second): (Vec<_>, Vec<_>) = input
        .iter()
        .map(|x| {
            let parts: Vec<&str> = x.split("   ").collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();
    first.sort();
    second.sort();
    iter::zip(first, second).map(|(a, b)| (a - b).abs()).sum()
}

fn p2(input: &Vec<String>) -> i32 {
    let mut distinct_count = HashMap::new();
    let first: Vec<i32> = input
        .iter()
        .scan(&mut distinct_count, |distinct_count, x| {
            let parts: Vec<&str> = x.split("   ").collect();
            let (a, b) = (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            );
            *distinct_count.entry(b).or_insert(0) += 1;
            Some(a)
        })
        .collect();
    first
        .into_iter()
        .map(|x| distinct_count.get(&x).unwrap_or(&0) * x)
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_in()?;
    write_out(p1(&input));
    write_out(p2(&input));
    Ok(())
}

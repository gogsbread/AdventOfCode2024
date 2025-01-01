use std::{cmp::Ordering, error::Error};

use aoc_2024::*;
use itertools::Itertools;

fn p1(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let rules = input
        .iter()
        .filter_map(|l| if l.contains('|') { Some(l) } else { None })
        .collect::<Vec<&String>>();
    input
        .iter()
        .filter(|l| l.contains(','))
        .map(|l| -> Result<i32, Box<dyn Error>> {
            let l: Vec<_> = l.split(',').collect();
            if l.iter().tuple_windows().all(|(u, v)| {
                let f = format!("{}|{}", u, v);
                rules.contains(&&f)
            }) {
                Ok(l[l.len() / 2].parse::<i32>()?)
            } else {
                Ok(0)
            }
        })
        .sum()
}

fn p2(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let rules = input
        .iter()
        .filter_map(|l| if l.contains('|') { Some(l) } else { None })
        .collect::<Vec<&String>>();

    input
        .iter()
        .filter(|l| l.contains(','))
        .map(|s| -> Result<i32, Box<dyn Error>> {
            let mut l: Vec<_> = s.split(',').collect();
            l.sort_by(|a, b| {
                if format!("{}|{}", a, b) == format!("{}|{}", b, a) {
                    Ordering::Equal
                } else if rules.contains(&&format!("{}|{}", a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            if l != s.split(',').collect::<Vec<_>>() {
                Ok(l[l.len() / 2].parse::<i32>()?)
            } else {
                Ok(0)
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_in()?;
    write_out(p1(&input)?);
    write_out(p2(&input)?);
    Ok(())
}

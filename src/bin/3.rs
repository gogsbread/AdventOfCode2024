use regex::Regex;
use std::error::Error;

use aoc_2024::*;

fn p1(input: &Vec<String>) -> Result<i64, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input
        .iter()
        .flat_map(|x| re.captures_iter(x))
        .map(|c| c.extract())
        .map(|(_, [x, y])| -> Result<i64, Box<dyn Error>> {
            Ok(x.parse::<i64>()? * y.parse::<i64>()?)
        })
        .sum()
}

fn p2(input: &Vec<String>) -> Result<i64, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    input
        .iter()
        .flat_map(|x| re.captures_iter(x))
        .scan(true, |enables, c| match c.get(0) {
            Some(m) => match m.as_str() {
                "don't()" => {
                    *enables = false;
                    Some(Ok(0))
                }
                "do()" => {
                    *enables = true;
                    Some(Ok(0))
                }
                _ => {
                    if *enables {
                        Some(Ok(c.get(1).unwrap().as_str().parse::<i64>().unwrap()
                            * c.get(2).unwrap().as_str().parse::<i64>().unwrap()))
                    } else {
                        Some(Ok(0))
                    }
                }
            },
            None => Some(Err(Box::from("No match"))),
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_in()?;
    write_out(p1(&input)?);
    write_out(p2(&input)?);
    Ok(())
}

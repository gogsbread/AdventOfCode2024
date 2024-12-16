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
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)")?;
    input
        .iter()
        .flat_map(|x| re.captures_iter(x))
        .scan(true, |enables, c| {
            let r: Result<i64, Box<dyn Error>> = (|| {
                let m = c.get(0).ok_or("No match found")?;
                match m.as_str() {
                    "don't()" => {
                        *enables = false;
                        Ok(0)
                    }
                    "do()" => {
                        *enables = true;
                        Ok(0)
                    }
                    _ => {
                        if *enables {
                            let a = c
                                .get(1)
                                .ok_or("Missing first group")?
                                .as_str()
                                .parse::<i64>()?;
                            let b = c
                                .get(2)
                                .ok_or("Missing second group")?
                                .as_str()
                                .parse::<i64>()?;
                            Ok(a * b)
                        } else {
                            Ok(0)
                        }
                    }
                }
            })();
            Some(r)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_in()?;
    write_out(p1(&input)?);
    write_out(p2(&input)?);
    Ok(())
}

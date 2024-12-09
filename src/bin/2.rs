use itertools::Itertools;
use std::error::Error;

use aoc_2024::*;

fn p1(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    input
        .iter()
        .map(|x| -> Result<i32, Box<dyn Error>> {
            let levels = x
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?;
            let decreasing = levels.iter().tuple_windows().all(|(a, b)| a < b);
            let increasing = levels.iter().tuple_windows().all(|(a, b)| a > b);
            let adjacent = levels
                .iter()
                .tuple_windows()
                .all(|(a, b)| (a - b).abs() <= 3 && (a - b).abs() >= 1);
            if (decreasing || increasing) && adjacent {
                Ok(1)
            } else {
                Ok(0)
            }
        })
        .sum()
}

fn p2(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    input
        .iter()
        .map(|x| -> Result<i32, Box<dyn Error>> {
            let levels = x
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?;
            let mut skip_indexes = std::iter::once(None).chain((0..levels.len()).map(Some));
            let success = skip_indexes.any(|skip_indexes| {
                let levels = levels
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if Some(i) == skip_indexes {
                            None
                        } else {
                            Some(*x)
                        }
                    })
                    .collect::<Vec<_>>();
                let decreasing = levels.iter().tuple_windows().all(|(a, b)| a < b);
                let increasing = levels.iter().tuple_windows().all(|(a, b)| a > b);
                let adjacent = levels
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| (a - b).abs() <= 3 && (a - b).abs() >= 1);
                (decreasing || increasing) && adjacent
            });
            if success {
                Ok(1)
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

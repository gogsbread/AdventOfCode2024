use std::{collections::HashSet, error::Error};

use aoc_2024::*;

fn patrol(
    (x, y, h): (i32, i32, char),
    obstacles: &Vec<(i32, i32)>,
) -> Result<(i32, i32, char), Box<dyn Error>> {
    match h {
        '^' => {
            if obstacles
                .iter()
                .find(|(a, b)| x - 1 == *a && y == *b)
                .is_some()
            {
                Ok((x, y, '>'))
            } else {
                Ok((x - 1, y, '^'))
            }
        }
        '>' => {
            if obstacles
                .iter()
                .find(|(a, b)| x == *a && y + 1 == *b)
                .is_some()
            {
                Ok((x, y, 'v'))
            } else {
                Ok((x, y + 1, '>'))
            }
        }
        'v' => {
            if obstacles
                .iter()
                .find(|(a, b)| x + 1 == *a && y == *b)
                .is_some()
            {
                Ok((x, y, '<'))
            } else {
                Ok((x + 1, y, 'v'))
            }
        }
        '<' => {
            if obstacles
                .iter()
                .find(|(a, b)| x == *a && y - 1 == *b)
                .is_some()
            {
                Ok((x, y, '^'))
            } else {
                Ok((x, y - 1, '<'))
            }
        }
        _ => Err("Invalid guard")?,
    }
}

fn p1(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut obstacles = Vec::new();
    let mut guard: (i32, i32, char) = (0, 0, '-');
    for (i, l) in input.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '^' {
                guard = (i as i32, j as i32, '^');
            }
            if c == '#' {
                obstacles.push((i as i32, j as i32));
            }
        }
    }
    assert_ne!(guard, (0, 0, '-'));

    let mut r: HashSet<(i32, i32)> = HashSet::from([(guard.0, guard.1)]);
    loop {
        guard = patrol(guard, &obstacles)?;
        if guard.0 < 0
            || guard.1 < 0
            || guard.0 as usize >= input.len()
            || guard.1 as usize >= input[0].len()
        {
            break;
        }
        r.insert((guard.0, guard.1));
    }
    Ok(r.len() as i32)
}

// fn p2(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
// draw a path
// for every point , place an obstacle and see if there is an existing
// }

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_in()?;
    write_out(p1(&input)?);
    // write_out(p2(&input)?);
    Ok(())
}

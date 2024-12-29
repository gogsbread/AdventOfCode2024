use std::error::Error;

use aoc_2024::*;

fn p1(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    const PAT: &str = "XMAS";

    let mut r = 0;
    let n = input.len();
    let m = input[0].len();
    for i in 0..n {
        for j in 0..m {
            let deltas: [(i32, i32); 8] = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (-1, -1),
                (1, 1),
                (-1, 1),
                (1, -1),
            ];
            r += deltas
                .map(|(dx, dy)| {
                    PAT.chars()
                        .scan((i, j), |(x, y), c| {
                            let r = input.get(*x)?.chars().nth(*y)? == c;
                            *x = (*x as i32 + dx) as usize;
                            *y = (*y as i32 + dy) as usize;
                            r.then_some(true)
                        })
                        .count()
                        == PAT.len()
                })
                .iter()
                .map(|x| *x as i32)
                .sum::<i32>();
        }
    }
    Ok(r)
}

fn p2(input: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    const PAT: &str = "MAS";

    let deltas = [
        [(-1, -1), (0, 0), (1, 1)],
        [(1, 1), (0, 0), (-1, -1)],
        [(-1, 1), (0, 0), (1, -1)],
        [(1, -1), (0, 0), (-1, 1)],
    ];
    let n = input.len();
    let m = input[0].len();
    Ok((0..m * n)
        .map(|k| {
            let i = k / m;
            let j = k % m;
            deltas
                .iter()
                .filter_map(|d| {
                    d.iter()
                        .enumerate()
                        .map(|(k, (dx, dy))| {
                            let x = (i as i32 + dx) as usize;
                            let y = (j as i32 + dy) as usize;
                            Some(input.get(x)?.chars().nth(y)? == PAT.chars().nth(k)?)
                        })
                        .all(|x| x.is_some() && x.unwrap())
                        .then_some(true)
                })
                .count()
                == 2
        })
        .map(|x| x as i32)
        .sum::<i32>())
    // filter all positions with A
    // check if check -1, -1 and 1,1 is MAS
    // check if 1,1 and -1-1 is MAS
    //is some(M) and 1,1 is some S
    // chef if
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_in()?;
    write_out(p1(&input)?);
    write_out(p2(&input)?);
    Ok(())
}

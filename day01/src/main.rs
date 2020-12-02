
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

static FILENAME: &str = "data/input";

fn read_input(fname: &str) -> Result<Vec<u32>, std::io::Error> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut result: Vec<u32> = Vec::new();

    for line in reader.lines() {
        result.push(line.unwrap().parse::<u32>().unwrap());
    }

    Ok(result)
}

fn part1_naive(s: &Vec<u32>) -> Result<u64, &'static str> {

    let mut result = Err("No result");

    for i in 0..s.len() {
        for j in 0..s.len() {
            if (i != j) && (s[i] + s[j] == 2020) {
                result = Ok(u64::from(s[i] * s[j]));
            }
        }

    }

    result
}

fn part2_naive(s: &Vec<u32>) -> Result<u64, &'static str> {

    let mut result = Err("No result");

    for i in 0..s.len() {
        for j in 0..s.len() {
            for k in 0..s.len() {
                if (i != j) && (i != k) && (j != k) && (s[i] + s[j] + s[k] == 2020) {
                    result = Ok(u64::from(s[i] * s[j] * s[k]));
                }
            }
        }

    }

    result
}

fn part1(s: &Vec<u32>) -> Result<u64, &'static str> {

    let v: Vec<u64> = s.iter()
                        .tuple_combinations::<(_, _)>()
                        .filter(|t| t.0 + t.1 == 2020)
                        .map(|t| u64::from(t.0 * t.1))
                        .take(1)
                        .collect();
    Ok(v[0])
}

fn part2(s: &Vec<u32>) -> Result<u64, &'static str> {

    let v: Vec<u64> = s.iter()
                        .tuple_combinations::<(_, _, _)>()
                        .filter(|t| t.0 + t.1 + t.2 == 2020)
                        .map(|t| u64::from(t.0 * t.1 * t.2))
                        .take(1)
                        .collect();
    Ok(v[0])
}


fn main() {

    let s = read_input(FILENAME).unwrap();

    println!("Part 1 naive answer = {}", part1_naive(&s).unwrap());
    println!("Part 1 answer       = {}", part1(&s).unwrap());
    println!("Part 2 naive answer = {}", part2_naive(&s).unwrap());
    println!("Part 2 answer       = {}", part2(&s).unwrap());

}

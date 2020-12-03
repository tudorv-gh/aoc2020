
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

static FILENAME: &str = "data/input";

fn read_input(fname: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut result: Vec<String> = Vec::new();

    for line in reader.lines() {
        result.push(line.unwrap());
    }

    Ok(result)
}

fn part1(s: &Vec<String>) -> Result<u32, &'static str> {
    let mut valid_passwd: u32 = 0;

    for line in s.iter() {

        let tokens: Vec<&str> = line.split_whitespace().collect();

        let range: Vec<usize> = tokens[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect();
        let letter = tokens[1].chars().next().unwrap();

        let count = tokens[2].chars().filter(|c| c == &letter).count();

        if (count >= range[0]) && (count <= range[1]) {
            valid_passwd += 1;
        }
    }

    Ok(valid_passwd)
}

fn part2(s: &Vec<String>) -> Result<u32, &'static str> {
    let mut valid_passwd: u32 = 0;

    for line in s.iter() {

        let tokens: Vec<&str> = line.split_whitespace().collect();

        let indices: Vec<usize> = tokens[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect();
        let letter = tokens[1].chars().next().unwrap();

        let pass: Vec<u8> = tokens[2].bytes().collect();

        if (pass[indices[0] - 1] as char == letter) ^ (pass[indices[1] - 1] as char == letter) {
            valid_passwd += 1;
        }
    }

    Ok(valid_passwd)
}

fn main() {

    let s = read_input(FILENAME).unwrap();

    println!("Part 1 answer       = {}", part1(&s).unwrap());
    println!("Part 2 answer       = {}", part2(&s).unwrap());
}

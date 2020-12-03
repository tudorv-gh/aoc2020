
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

static FILENAME: &str = "data/input";

fn read_input(fname: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut result = Vec::new();

    for line in reader.lines() {
        result.push(line.unwrap());
    }

    Ok(result)
}

fn part1(s: &Vec<String>, slope_r: usize, slope_d: usize) -> Result<u32, &'static str> {
    let mut tree_count: u32 = 0;
    let mut hpos: usize = 0;
    let mut vpos: usize = 0;

    let hsize = s[0].len();

    loop {
        vpos += slope_d;
        if vpos >= s.len() {
            break;
        }

        hpos += slope_r;
        hpos = hpos % hsize;

        let c = s[vpos].chars().nth(hpos).unwrap();
        if c.to_string() == "#" {
            tree_count += 1;
        }
    }

    Ok(tree_count)
}

fn part2(s: &Vec<String>) -> Result<u32, &'static str> {

    let mut result = 1;
    result *= part1(s, 1, 1).unwrap();
    result *= part1(s, 3, 1).unwrap();
    result *= part1(s, 5, 1).unwrap();
    result *= part1(s, 7, 1).unwrap();
    result *= part1(s, 1, 2).unwrap();

    Ok(result)
}

fn main() {
    let s = read_input(FILENAME).unwrap();

    println!("Part 1 answer       = {}", part1(&s, 3, 1).unwrap());
    println!("Part 2 answer       = {}", part2(&s).unwrap());

}

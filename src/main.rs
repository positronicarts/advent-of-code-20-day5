use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

struct BoardingPass {
    value: usize,
}

impl BoardingPass {
    fn get_value(&self) -> usize {
        self.value
    }
}

impl FromStr for BoardingPass {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;
        let mut fac = 1;
        let iter = s.chars().rev().collect::<Vec<char>>();

        for c in iter.iter() {
            if (*c == 'B') || (*c == 'R') {
                value += fac;
            }
            fac *= 2;
        }

        Ok(BoardingPass { value })
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut bps = get_bps(opts.input);
    if opts.part == 1 {
        let max = bps.iter().map(|x| x.get_value()).max();
        println!("Max {}", max.unwrap());
    } else {
        bps.sort_by_key(|x| x.get_value());
        let mut current = None;
        for bp in bps.iter().map(BoardingPass::get_value) {
            if let Some(c) = current {
                if bp != c + 1 {
                    println!("My seat is {}", c + 1);
                    break;
                }
            }
            current = Some(bp);
        }
    }
}

fn get_bps(filename: String) -> Vec<BoardingPass> {
    let mut bps = Vec::<BoardingPass>::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_as_string) = line {
                bps.push(BoardingPass::from_str(&line_as_string).unwrap())
            }
        }
    }

    bps
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

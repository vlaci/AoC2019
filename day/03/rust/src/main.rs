const USAGE: &str = "
Day 3: Crossed Wires

Usage: wires <input-file>
       wires --help

Options:
    -h, --help  Print this help.
";

use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    multi::separated_list,
    sequence::pair,
    IResult,
};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use docopt::Docopt;

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    let input_file = args.get_str("<input-file>");
}

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq)]
struct Step {
    direction: Direction,
    length: u8,
}

impl Step {
    fn new(direction: Direction, length: u8) -> Step {
        Step { direction, length }
    }
}

fn parse_path(path: &str) -> IResult<&str, Vec<Step>> {
    separated_list(
        tag(","),
        map(
            pair(
                map_res(take(1u8), |d| match d {
                    "U" => Ok(Direction::UP),
                    "D" => Ok(Direction::DOWN),
                    "L" => Ok(Direction::LEFT),
                    "R" => Ok(Direction::RIGHT),
                    _ => Err("Invalid direction"),
                }),
                map_res(take(1u8), |d: &str| d.parse()),
            ),
            |d| Step::new(d.0, d.1),
        ),
    )(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_parsing() {
        assert_eq!(
            parse_path("R8,U5,L5,D3").unwrap().1,
            vec![
                Step::new(Direction::RIGHT, 8),
                Step::new(Direction::UP, 5),
                Step::new(Direction::LEFT, 5),
                Step::new(Direction::DOWN, 3)
            ]
        );
    }
}

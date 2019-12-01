const USAGE: &str = "
Day 1: The Tyranny of the Rocket Equation

Usage: fuel <input-file>
       fuel --help

Options:
    -h, --help  Print this help.
";

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

extern crate docopt;
use docopt::Docopt;

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    let input_file = args.get_str("<input-file>");
    fuel(input_file);
}

fn fuel(input_file: &str) {
    let file = File::open(input_file).expect("unable to open file");
    let inputs: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.expect("invalid file encoding")
                .parse()
                .expect("invalid integer literal")
        })
        .collect();

    let result = inputs.iter().map(|f| get_required_fuel(*f)).sum::<i32>();
    println!("The required fuel for all the parts is: {}", result);

    let result = inputs
        .iter()
        .map(|f| get_required_fuel_exp(*f))
        .sum::<i32>();
    println!(
        "The required fuel for all the parts and fuel is: {}",
        result
    );
}

fn get_required_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_required_fuel_exp(mass: i32) -> i32 {
    let mut total = 0;
    let mut f = mass;
    loop {
        f = get_required_fuel(f);
        if f <= 0 {
            break;
        }

        total += f;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_fuel() {
        /* For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
           For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
           For a mass of 1969, the fuel required is 654.
           For a mass of 100756, the fuel required is 33583.
        */
        assert_eq!(get_required_fuel(12), 2);
        assert_eq!(get_required_fuel(14), 2);
        assert_eq!(get_required_fuel(1969), 654);
        assert_eq!(get_required_fuel(100756), 33583);
    }

    #[test]
    fn test_required_fuel_exp() {
        /* A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0,
           which would call for a negative fuel), so the total fuel required is still just 2.
           At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2).
           216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel.
           So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
           The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
        */
        assert_eq!(get_required_fuel_exp(14), 2);
        assert_eq!(get_required_fuel_exp(1969), 966);
        assert_eq!(get_required_fuel_exp(100756), 50346);
    }
}

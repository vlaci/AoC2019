const USAGE: &str = "
Day 2: 1202 Program Alarm

Usage: alarm <input-file>
       alarm --help

Options:
    -h, --help  Print this help.
";

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use docopt::Docopt;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    let input_file = args.get_str("<input-file>");
    alarm(input_file);
}

fn alarm(input_file: &str) {
    let file = File::open(input_file).expect("unable to open file");
    let mut inputs: Vec<usize> = BufReader::new(file)
        .split(b',')
        .map(|num| {
            std::str::from_utf8(&num.unwrap())
                .expect("invalid file encoding")
                .trim()
                .parse()
                .expect("invalid integer literal")
        })
        .collect();

    inputs[1] = 12;
    inputs[2] = 02;

    let result = run_program(inputs.as_slice());

    println!("The value left at position 0 is: {}", result[0]);

    'l: for i in 0..100 {
        for j in 0..100 {
            inputs[1] = i;
            inputs[2] = j;

            let result = run_program(inputs.as_slice());
            if result[0] == 19690720 {
                println!("The input needed is: {}", i*100+j);
                break 'l;
            }
        }
    }

}

#[derive(FromPrimitive)]
enum OpCodes {
    ADD = 1,
    MUL = 2,
    HALT = 99,
}

fn run_program(code: &[usize]) -> Vec<usize> {
    let mut code = Vec::from(code);
    let mut ip = 0;

    loop {
        let op = OpCodes::from_usize(code[ip]).expect("Invalid opcode");
        match op {
            OpCodes::ADD => {
                let op1 = code[ip+1];
                let op2 = code[ip+2];
                let store_at = code[ip+3];
                code[store_at] = code[op1] + code[op2];
                ip += 4;
            },
            OpCodes::MUL => {
                let op1 = code[ip+1];
                let op2 = code[ip+2];
                let store_at = code[ip+3];
                code[store_at] = code[op1] * code[op2];
                ip += 4;
            },
            OpCodes::HALT => break, 
        }
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode() {
        /*  1,9,10,3,2,3,11,0,99,30,40,50
            1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
            2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
            2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
            1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        */
        assert_eq!(run_program(&[1,0,0,0,99]), [2,0,0,0,99]);
        assert_eq!(run_program(&[2,3,0,3,99]), [2,3,0,6,99]);
        assert_eq!(run_program(&[2,4,4,5,99,0]), [2,4,4,5,99,9801]);
        assert_eq!(run_program(&[1,1,1,4,99,5,6,0,99]), [30,1,1,4,2,5,6,0,99]);
        assert_eq!(run_program(&[1,9,10,3,2,3,11,0,99,30,40,50]), [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}

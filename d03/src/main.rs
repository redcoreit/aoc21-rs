use std::{
    io::{BufRead, BufReader},
};
use core::cmp::Ordering;

struct DiagData {
    gamma_rate : u32,
    epsilon_rate : u32,
}

impl DiagData {
    fn get_power_cons(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

fn main() {
    // 000001110001
    // 000001111101
    // 010011000000
    // 000000000011
    // 010000100111

    let lines = get_input_lines();
    let lines_len = lines.len();
    let digits_len: usize = lines[0].trim().chars().count();

    // println!("digits: {}", digits);
    if digits_len > 32 {
        panic!("Too many digits per line.");
    }

    let sum = get_sum_by_digits(lines, digits_len);    
    // println!("Half: {}", half);

    let diag = get_diag_data(sum, digits_len, lines_len);

    println!("gamma rate: {}", diag.gamma_rate);
    println!("epsilon rate: {}", diag.epsilon_rate);
    println!("power consumption: {}", diag.get_power_cons());
}

fn get_input_lines() -> Vec<String> {
    let cwd = &std::env::current_dir().unwrap();
    let cwd = std::path::Path::new(cwd);
    let file_path = cwd.join("input.txt");
    let file = std::fs::File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|m| m.unwrap()).collect();

    lines
}

fn get_sum_by_digits(lines : Vec<String> , digits_len : usize) -> Vec<u32> {
    // sum of lines by digits
    let mut sum: Vec<u32> = vec![0; digits_len ];

    for line in lines {
        // println!("{}", line);
        for (i, digit) in sum.iter_mut().enumerate() {
            let chr = &line.chars().nth(i as usize).unwrap();
            let num = chr.to_digit(10).unwrap();

            let inc = match num {
                0 => 0,
                1 => 1,
                _ => panic!("Invalid digit."),
            };

            *digit += inc;
        }
    }

    sum
}

fn get_diag_data(sum : Vec<u32>, digits_len : usize, lines_len : usize) -> DiagData {
    let half = lines_len as f32 / 2.0;

    // gamma rate = most common bits
    let mut gamma_rate = 0;

    // epsilon rate = least common bits
    let mut epsilon_rate = 0;

    for (i, digit_sum) in sum.iter().enumerate() {

        let shift = digits_len - 1 - i;
        let digit_sum = *digit_sum as f32;

        // println!("index: {}, shift: {}", i, shift);

        match digit_sum.partial_cmp(&half).unwrap() {
            Ordering::Less => {
                epsilon_rate |= 1 << shift;
                // println!("epsilon_rate: {:#014b}", epsilon_rate);
            }
            Ordering::Greater => {
                gamma_rate |= 1 << shift;
                // println!("gamma_rate: {:#014b}", gamma_rate);
            }
            Ordering::Equal => {
                panic!("Zero and one count is equal at index {}.", i);
            }
        }
    }

    DiagData { gamma_rate,  epsilon_rate }
}

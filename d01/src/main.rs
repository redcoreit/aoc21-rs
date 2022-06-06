use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

enum DiffType {
    NA,
    Inc,
    Dec
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let path = Path::new(&cwd).join("input.txt"); 

    let lines = read_file(&path);
    let nums = cast_to_num(lines);
    let diffs = get_relative_diffs(nums);

    let sum_of_incs = diffs.into_iter().fold(0u32, |acc, m| {
        let val = match m {
            DiffType::Inc => 1,
            _ => 0,
        };

        acc + val
    });

    println!("{}", sum_of_incs);
}

fn read_file(filename: &PathBuf) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines
}

fn cast_to_num(lines: Lines<BufReader<File>>) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new(); 

    for line in lines {
        let num: u32 = line.unwrap().parse().unwrap();
        nums.push(num);
    }

    nums
}

fn get_relative_diffs(nums: Vec<u32>) -> Vec<DiffType> {
    let mut prev : Option<u32> = Option::None; 
    let mut diffs: Vec<DiffType> = Vec::new();

    for num in nums {
        let diff = match prev {
            None => DiffType::NA,
            Some(value) => {
                if num > value {
                    DiffType::Inc
                }
                else {
                    DiffType::Dec
                }
            }
        };

        prev = Some(num);
        diffs.push(diff);
    }

    diffs
}

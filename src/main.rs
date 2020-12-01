use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let numbers = read_numbers();
    let answer1 = findMul2(&numbers);
    println!("Answer 1: {}", answer1);
    let answer2 = findMul3(&numbers);
    println!("Answer 2: {}", answer2);
}

fn findMul2(numbers: &Vec<u64>) -> u64 {
    let last = numbers.len() - 1;
    for first in 1 .. last {
        for second in first.. last {
            let num1 = numbers[first];
            let num2 = numbers[second];
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    panic!("Found no matches")
}

fn findMul3(numbers: &Vec<u64>) -> u64 {
    let last = numbers.len() - 1;
    for first in 1 .. last {
        for second in first.. last {
            for third in second .. last {
                let num1 = numbers[first];
                let num2 = numbers[second];
                let num3 = numbers[third];
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    panic!("Found no matches")
}

fn read_numbers() -> Vec<u64> {
    let mut numbers: Vec<u64> = vec!();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(s) = line {
                let num: u64 = s.parse().unwrap();
                numbers.push(num);
            }
        }
    }
    return numbers;
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

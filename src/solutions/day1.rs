use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn calculate_calibration(data: &File) -> u32 {
    BufReader::new(data)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            digits[0] * 10 + digits.last().unwrap_or(&0)
        })
        .sum()
}

pub fn run_day1() -> io::Result<()> {
    let file = File::open("input/day1.txt")?;
    let sum_part1 = calculate_calibration(&file);

    println!("Day 1 - Part 1: {}", sum_part1);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_calibration() {
        let data = File::open("input/day1_test.txt").unwrap();     
        assert_eq!(calculate_calibration(&data), 142);
    }

    #[test]
    fn test_run_day1() {
        assert!(run_day1().is_ok());
    }
}



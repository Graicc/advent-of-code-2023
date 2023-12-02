use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter_map(|c| c.to_digit(10));
            let first = nums.next().unwrap();
            first * 10 + nums.last().unwrap_or(first)
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut map = HashMap::new();
    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map.insert("0", 0);
    map.insert("1", 1);
    map.insert("2", 2);
    map.insert("3", 3);
    map.insert("4", 4);
    map.insert("5", 5);
    map.insert("6", 6);
    map.insert("7", 7);
    map.insert("8", 8);
    map.insert("9", 9);

    input
        .lines()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            let mut chars = line.chars();
            loop {
                for (k, v) in &map {
                    if chars.as_str().starts_with(k) {
                        first = Some(first.unwrap_or(v));
                        last = Some(v);
                    }
                }
                if let None = chars.next() {
                    break;
                }
            }

            first.unwrap() * 10 + last.unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(142, part_one(input));
    }

    #[test]
    fn test_2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        assert_eq!(281, part_two(input));
    }
}

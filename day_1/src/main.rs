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
    let nums = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    input
        .lines()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            let mut chars = line.chars();

            loop {
                for (k, v) in nums {
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

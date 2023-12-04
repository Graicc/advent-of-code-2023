fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split([':', '|']);
            let _ = parts.next().unwrap(); // Card + #
            let winnings: Vec<u32> = parts
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .collect();

            let count = parts
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .filter(|num| winnings.contains(num))
                .count() as u32;

            if count > 0 {
                2_u32.pow(count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let win_counts: Vec<usize> = input
        .lines()
        .map(|line| {
            let mut parts = line.split([':', '|']);
            let _ = parts.next().unwrap(); // Card + #
            let winnings: Vec<u32> = parts
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .collect();

            parts
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .filter(|num| winnings.contains(num))
                .count()
        })
        .collect();

    let mut card_counts: Vec<u32> = vec![1; win_counts.len()];

    for i in 0..card_counts.len() {
        let wins = win_counts[i];
        let cards = card_counts[i];

        for j in 0..wins {
            card_counts[i + j + 1] += cards;
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = include_str!("sample");

        assert_eq!(13, part_one(input));
    }

    #[test]
    fn test_two() {
        let input = include_str!("sample2");

        assert_eq!(30, part_two(input));
    }
}

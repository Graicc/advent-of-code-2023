use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn contains_around(set: &HashSet<(usize, usize)>, i: usize, j: usize) -> bool {
    [
        (Some(i), Some(j)),
        (Some(i), j.checked_add(1)),
        (Some(i), j.checked_sub(1)),
        (i.checked_add(1), Some(j)),
        (i.checked_add(1), j.checked_add(1)),
        (i.checked_add(1), j.checked_sub(1)),
        (i.checked_sub(1), Some(j)),
        (i.checked_sub(1), j.checked_add(1)),
        (i.checked_sub(1), j.checked_sub(1)),
    ]
    .into_iter()
    .filter_map(|x| match x {
        (Some(i), Some(j)) => Some((i, j)),
        _ => None,
    })
    .any(|x| set.contains(&x))
}

fn part_one(input: &str) -> u32 {
    let symbols: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.' && !c.is_digit(10))
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut good = false;
        for (j, c) in line.chars().chain(['.']).enumerate() {
            match c.to_digit(10) {
                Some(digit) => {
                    num *= 10;
                    num += digit;

                    good |= contains_around(&symbols, i, j);
                }
                None => {
                    if good {
                        sum += num;
                    }
                    num = 0;
                    good = false;
                }
            }
        }
    }

    sum
}

fn contains_around_2(set: &HashSet<(usize, usize)>, i: usize, j: usize) -> Vec<(usize, usize)> {
    [
        (Some(i), Some(j)),
        (Some(i), j.checked_add(1)),
        (Some(i), j.checked_sub(1)),
        (i.checked_add(1), Some(j)),
        (i.checked_add(1), j.checked_add(1)),
        (i.checked_add(1), j.checked_sub(1)),
        (i.checked_sub(1), Some(j)),
        (i.checked_sub(1), j.checked_add(1)),
        (i.checked_sub(1), j.checked_sub(1)),
    ]
    .into_iter()
    .filter_map(|x| match x {
        (Some(i), Some(j)) => Some((i, j)),
        _ => None,
    })
    .filter(|x| set.contains(&x))
    .collect()
}

fn part_two(input: &str) -> u32 {
    let symbols: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let mut found: HashMap<(usize, usize), u32> = HashMap::new();
    let mut complete: HashMap<(usize, usize), u32> = HashMap::new();

    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut gears = HashSet::new();
        for (j, c) in line.chars().chain(['.']).enumerate() {
            match c.to_digit(10) {
                Some(digit) => {
                    num *= 10;
                    num += digit;
                    for gear in contains_around_2(&symbols, i, j) {
                        gears.insert(gear);
                    }
                }
                None => {
                    for gear in gears {
                        if let Some(comp) = complete.get_mut(&gear) {
                            // more than 2
                            sum -= *comp;
                            *comp = 0;
                            continue;
                        }

                        if let Some(&other) = found.get(&gear) {
                            // we are second
                            let total = num * other;
                            sum += total;
                            found.remove(&gear);
                            complete.insert(gear, total);
                        } else {
                            // we are first
                            found.insert(gear, num);
                        }
                    }

                    num = 0;
                    gears = HashSet::new();
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("sample");

        assert_eq!(4361, part_one(input));
        assert_eq!(467835, part_two(input));
    }
}

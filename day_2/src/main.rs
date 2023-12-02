fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    // println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(&[' ', ':', ',', ';']).filter(|x| !x.is_empty());

            parts.next(); // Game
            let id = parts.next()?.parse::<u32>().ok()?;

            while let Some(num) = parts.next().and_then(|x| x.parse::<u32>().ok()) {
                match parts.next() {
                    Some("red") => {
                        if num > 12 {
                            return None;
                        }
                    }
                    Some("green") => {
                        if num > 13 {
                            return None;
                        }
                    }
                    Some("blue") => {
                        if num > 14 {
                            return None;
                        }
                    }
                    _ => return None,
                }
            }

            Some(id)
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(&[' ', ':', ',', ';']).filter(|x| !x.is_empty());

            parts.next(); // Game
            parts.next(); // Game #

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            while let Some(num) = parts.next().and_then(|x| x.parse::<u32>().ok()) {
                match parts.next() {
                    Some("red") => {
                        max_red = max_red.max(num);
                    }
                    Some("green") => {
                        max_green = max_green.max(num);
                    }
                    Some("blue") => {
                        max_blue = max_blue.max(num);
                    }
                    _ => {}
                }
            }

            max_red * max_green * max_blue
        })
        .sum()
}

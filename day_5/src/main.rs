use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    // println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

impl Mapping {
    fn apply(&self, idx: u64) -> Option<u64> {
        if idx >= self.src_start && idx < self.src_start + self.len {
            Some(idx + self.dest_start - self.src_start)
        } else {
            None
        }
    }
}

fn map(mappings: &Vec<Mapping>, idx: u64) -> u64 {
    let mut min = 0;
    let mut max = mappings.len();

    // Binary sort in real life :0
    while min < max {
        let mid = (min + max) / 2;

        let curr = &mappings[mid];

        if let Some(res) = curr.apply(idx) {
            return res;
        }

        if curr.src_start < idx {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    idx
}

fn part_one(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|num| num.parse::<u64>().ok());

    let mut mappings: Vec<Vec<Mapping>> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            // new part
            lines.next().unwrap(); // title
            mappings.push(Vec::new());
            continue;
        }

        let mut nums = line.split(' ').filter_map(|num| num.parse::<u64>().ok());
        let dest_start = nums.next().unwrap();
        let src_start = nums.next().unwrap();
        let len = nums.next().unwrap();

        mappings.last_mut().unwrap().push(Mapping {
            dest_start,
            src_start,
            len,
        });
    }

    for mapping in &mut mappings {
        mapping.sort_by(|a, b| a.src_start.cmp(&b.src_start));
    }

    seeds
        .map(|mut seed| {
            for mapping in &mappings {
                seed = map(mapping, seed);
            }
            seed
        })
        .min()
        .unwrap()
}

fn part_two(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|num| num.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let seeds: Vec<_> = seeds.chunks(2).collect();
    let seeds = seeds.par_iter().flat_map(|x| x[0]..(x[0] + x[1]));

    let mut mappings: Vec<Vec<Mapping>> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            // new part
            lines.next().unwrap(); // title
            mappings.push(Vec::new());
            continue;
        }

        let mut nums = line.split(' ').filter_map(|num| num.parse::<u64>().ok());
        let dest_start = nums.next().unwrap();
        let src_start = nums.next().unwrap();
        let len = nums.next().unwrap();

        mappings.last_mut().unwrap().push(Mapping {
            dest_start,
            src_start,
            len,
        });
    }

    for mapping in &mut mappings {
        mapping.sort_by(|a, b| a.src_start.cmp(&b.src_start));
    }

    seeds
        .map(|mut seed| {
            for mapping in &mappings {
                seed = map(mapping, seed);
            }
            seed
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("sample");

        assert_eq!(35, part_one(input));
        assert_eq!(46, part_two(input));
    }
}

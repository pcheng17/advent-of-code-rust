pub struct Range {
    a: u32,
    b: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.a <= other.a && self.b >= other.b
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.b >= other.a && other.b >= self.a
    }
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .filter_map(|line| {
            let (r1, r2) = line.split_once(',')?;
            let (r1_a, r1_b) = r1.split_once('-')?;
            let (r2_a, r2_b) = r2.split_once('-')?;
            Some((
                Range {
                    a: r1_a.parse::<u32>().ok()?,
                    b: r1_b.parse::<u32>().ok()?,
                },
                Range {
                    a: r2_a.parse::<u32>().ok()?,
                    b: r2_b.parse::<u32>().ok()?,
                },
            ))
        })
        .collect()
}

pub fn part_one(input: &[(Range, Range)]) -> Option<usize> {
    Some(
        input
            .iter()
            .filter(|(a, b)| a.contains(b) || b.contains(a))
            .count(),
    )
}

pub fn part_two(input: &[(Range, Range)]) -> Option<usize> {
    Some(input.iter().filter(|(a, b)| a.overlaps(b)).count())
}

fn main() {
    let input = advent_of_code::read_file("inputs", 4);
    let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        let parsed = parse(&input);
        assert_eq!(part_one(&parsed), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        let parsed = parse(&input);
        assert_eq!(part_two(&parsed), Some(4));
    }
}

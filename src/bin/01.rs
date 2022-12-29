use itertools::Itertools;

fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|x| x.lines().filter_map(|n| n.parse::<u32>().ok()).sum())
        .collect::<Vec<u32>>()
}

pub fn part_one(calories: &[u32]) -> Option<u32> {
    calories.iter().max().copied()
}

pub fn part_two(calories: &[u32]) -> Option<u32> {
    Some(
        calories
            .iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<u32>(),
    )
}

fn main() {
    let input = advent_of_code::read_file("inputs", 1);
    let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        let calories = parse(&input);
        assert_eq!(part_one(&calories), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        let calories = parse(&input);
        assert_eq!(part_two(&calories), Some(45000));
    }
}

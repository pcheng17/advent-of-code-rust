fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(' ')?;
            Some((
                (match a {
                    "A" => 0_u32,
                    "B" => 1_u32,
                    "C" => 2_u32,
                    &_ => unreachable!(),
                }),
                (match b {
                    "X" => 0_u32,
                    "Y" => 1_u32,
                    "Z" => 2_u32,
                    &_ => unreachable!(),
                }),
            ))
        })
        .collect()
}

pub fn part_one(input: &[(u32, u32)]) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|(other, ours)| {
                // Had to do some silly arithmetic to make sure the input of the modulo operator
                // is positive. Otherwise, Rust's modulo operator will keep the sign of the input.
                let score = ((3 - ((2 + other - ours) % 3)) % 3) * 3;
                score + ours + 1
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &[(u32, u32)]) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|(other, result)| {
                let ours = match result {
                    0 => (other + 2) % 3,
                    1 => *other,
                    2 => (other + 1) % 3,
                    _ => unreachable!(),
                };
                let score = ((3 - ((2 + other - ours) % 3)) % 3) * 3;
                score + ours + 1
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = advent_of_code::read_file("inputs", 2);
    let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        let parsed = parse(&input);
        assert_eq!(part_one(&parsed), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        let parsed = parse(&input);
        assert_eq!(part_two(&parsed), Some(12));
    }
}

use itertools::Itertools;

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn find_marker(chars: &[char], window_size: usize) -> Option<usize> {
    chars
        .windows(window_size)
        .find_position(|chars| {
            for (i, x) in chars.iter().enumerate() {
                for (j, y) in chars.iter().enumerate() {
                    if i != j && x == y {
                        return false;
                    }
                }
            }
            true
        })
        .map(|(pos, _)| pos + window_size)
}

pub fn part_one(input: &[char]) -> Option<usize> {
    find_marker(input, 4)
}

pub fn part_two(input: &[char]) -> Option<usize> {
    find_marker(input, 14)
}

fn main() {
    let input = advent_of_code::read_file("inputs", 6);
    let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let parsed = parse(&input);
        assert_eq!(part_one(&parsed), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let parsed = parse(&input);
        assert_eq!(part_two(&parsed), None);
    }
}

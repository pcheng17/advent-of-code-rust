pub fn part_one(input: &str) -> Option<u32> {
    for line in input.split("\n") {
        println!("{}", line);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = advent_of_code::read_file("inputs", 3);
    // let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}

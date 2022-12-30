// use std::collections::HashSet;

// fn get_priority(code: u8) -> u8 {
//     code % 32 + (26 * (code <= 90) as u8)
// }

pub fn part_one(_input: &str) -> Option<u32> {
    // input
    //     .lines()
    //     .map(|l| {
    //         let parts = l.split_at(l.len() / 2);
    //         // let a: HashSet<&u8> = HashSet::from_iter(parts.0.as_bytes().iter());
    //         // let b: HashSet<&u8> = HashSet::from_iter(parts.1.as_bytes().iter());
    //         let a = parts.0.as_bytes();
    //         let b = parts.1.as_bytes();
    //         l.len()
    //     }).sum::<usize>();
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
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

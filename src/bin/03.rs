fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_priority(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}

pub fn part_one(input: &Vec<&str>) -> Option<u32> {
    Some(input
        .iter()
        .filter_map(|&l| {
            let parts = l.split_at(l.len() / 2);
            let a = parts.0.as_bytes();
            let b = parts.1.as_bytes();
            a.iter()
                .find(|code| b.contains(code))
                .map(|&code| get_priority(code) as u32)
        }).sum::<u32>())
}

pub fn part_two(input: &Vec<&str>) -> Option<u32> {
    Some(input
        .chunks(3)
        .filter_map(|chunk| {
            let mut it = chunk.iter();
            let a = it.next()?.as_bytes();
            let b = it.next()?.as_bytes();
            let c = it.next()?.as_bytes();
            a.iter()
                .find(|code| b.contains(code) && c.contains(code))
                .map(|&code| get_priority(code) as u32)
        })
        .sum::<u32>())
}

fn main() {
    let input = advent_of_code::read_file("inputs", 3);
    let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        let parsed = advent_of_code::run_parser(parse, &input);
        assert_eq!(part_one(&parsed), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        let parsed = advent_of_code::run_parser(parse, &input);
        assert_eq!(part_two(&parsed), None);
    }
}

fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

fn find_marker(input: &[u8], window_size: usize) -> Option<usize> {
    let mut mask = 0u32;
    input
        .iter()
        .take(window_size - 1)
        .for_each(|c| mask ^= 1 << (c % 32));

    input
        .windows(window_size)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            mask ^= 1 << (last % 32);
            let res = mask.count_ones() == window_size as u32;
            mask ^= 1 << (first % 32);
            res
        })
        .map(|i| i + window_size)
}

pub fn part_one(input: &[u8]) -> Option<usize> {
    find_marker(input, 4)
}

pub fn part_two(input: &[u8]) -> Option<usize> {
    find_marker(input, 14)
}

fn main() {
    let input = advent_of_code::read_file("inputs", 6);
    let parsed = advent_of_code::run_parser(parse, &input);
    advent_of_code::solve!(1, part_one, parsed);
    advent_of_code::solve!(2, part_two, parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let parsed = parse(&input);
        assert_eq!(part_one(&parsed), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let parsed = parse(&input);
        assert_eq!(part_two(&parsed), Some(29));
    }
}

use std::cmp;
use std::ops::Index;

#[derive(Copy, Clone)]
enum Type {
    Ore,
    Clay ,
    Obsidian,
    Geode,
}

pub struct Blueprint {
    id: u16,
    costs: [[u16; 3]; 4],
}

#[derive(Copy, Clone)]
struct State {
    robots: [u32; 4],
    resources: [u32; 4],
    time_left: u32,
}

// Allows instances of `Type` to be used as indices into `[u32; 4]`
impl Index<Type> for [u32; 4] {
    type Output = u32;

    fn index(&self, resource: Type) -> &Self::Output {
        match resource {
            Type::Ore => &self[0],
            Type::Clay => &self[1],
            Type::Obsidian => &self[2],
            Type::Geode => &self[3],
        }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let id = words
                .nth(1)
                .unwrap()
                .trim_end_matches(':')
                .parse::<u16>()
                .unwrap();
            let costs: [[u16; 3]; 4] = [
                [words.nth(4).unwrap().parse::<u16>().unwrap(), 0, 0],
                [words.nth(5).unwrap().parse::<u16>().unwrap(), 0, 0],
                [
                    words.nth(5).unwrap().parse::<u16>().unwrap(),
                    words.nth(2).unwrap().parse::<u16>().unwrap(),
                    0,
                ],
                [
                    words.nth(5).unwrap().parse::<u16>().unwrap(),
                    0,
                    words.nth(2).unwrap().parse::<u16>().unwrap(),
                ],
            ];
            Blueprint { id, costs: costs }
        })
        .collect()
}

fn optimistic_best(state: &State, resource: Type) -> u32 {
    let t = state.time_left;
    state.resources[resource] + state.robots[resource] * t + (t * (t - 1) / 2)
}

fn get_max_geodes(blueprint: &Blueprint, state: &State, best: u32) -> u32 {
    if state.time_left == 1 {
        return state.resources[Type::Geode] + state.robots[Type::Geode];
    }

    if optimistic_best(state, Type::Geode) < best {
        return 0;
    }

    let mut new_state = *state;
    new_state.time_left -= 1;

    return 0;
}

pub fn part_one(input: &Vec<Blueprint>) -> Option<u32> {
    None
}

pub fn part_two(input: &Vec<Blueprint>) -> Option<u32> {
    None
}

fn main() {
    let input = advent_of_code::read_file("inputs", 19);
    let parsed = advent_of_code::run_parser(parse, &input);
    // for bp in &parsed {
    //     println!("{} {} {}", bp.costs[0][0], bp.costs[0][1], bp.costs[0][2]);
    // }
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        let parsed = parse(&input);
        assert_eq!(part_one(&parsed), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        let parsed = parse(&input);
        assert_eq!(part_two(&parsed), None);
    }
}

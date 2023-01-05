use rayon::prelude::*;

#[derive(Copy, Clone)]
enum Type {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Debug)]
pub struct Blueprint {
    id: u32,
    costs: [[u32; 3]; 4],
    max_costs: [u32; 4],
}

fn get_max_costs(costs: &[[u32; 3]; 4]) -> [u32; 4] {
    let mut max_costs = [0, 0, 0, u32::MAX];
    for recipe in costs {
        for (i, &x) in recipe.iter().enumerate() {
            max_costs[i] = std::cmp::max(max_costs[i], x);
        }
    }
    max_costs
}

impl Blueprint {
    fn new(id: u32, costs: [[u32; 3]; 4]) -> Self {
        let max_costs = get_max_costs(&costs);
        Self {
            id,
            costs,
            max_costs,
        }
    }
}

#[derive(Copy, Clone)]
struct State {
    robots: [u32; 4],
    resources: [u32; 4],
    time_left: u32,
}

impl State {
    fn can_build(&self, blueprint: &Blueprint, rtype: Type) -> bool {
        let maxed = self.robots[rtype as usize] >= blueprint.max_costs[rtype as usize];
        let recipe = &blueprint.costs[rtype as usize];
        let can_afford = recipe
            .iter()
            .enumerate()
            .all(|(i, &x)| self.resources[i] >= x);
        !maxed && can_afford
    }

    #[allow(dead_code)]
    fn can_fund(&self, blueprint: &Blueprint, rtype: Type) -> bool {
        let recipe = &blueprint.costs[rtype as usize];
        recipe.iter().enumerate().all(|(i, &x)| self.robots[i] >= x)
    }

    fn gather(&mut self) {
        for (x, y) in self.resources.iter_mut().zip(&self.robots) {
            *x += y;
        }
    }

    fn build(&mut self, blueprint: &Blueprint, rtype: Type) {
        let recipe = &blueprint.costs[rtype as usize];
        for (i, x) in recipe.iter().enumerate() {
            self.resources[i] -= x;
        }
        self.robots[rtype as usize] += 1;
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
                .parse::<u32>()
                .unwrap();
            let costs: [[u32; 3]; 4] = [
                [words.nth(4).unwrap().parse::<u32>().unwrap(), 0, 0],
                [words.nth(5).unwrap().parse::<u32>().unwrap(), 0, 0],
                [
                    words.nth(5).unwrap().parse::<u32>().unwrap(),
                    words.nth(2).unwrap().parse::<u32>().unwrap(),
                    0,
                ],
                [
                    words.nth(5).unwrap().parse::<u32>().unwrap(),
                    0,
                    words.nth(2).unwrap().parse::<u32>().unwrap(),
                ],
            ];
            Blueprint::new(id, costs)
        })
        .collect()
}

fn optimistic_best(state: &State, rtype: Type, time_left: Option<u32>) -> u32 {
    let t = time_left.unwrap_or(state.time_left);
    state.resources[rtype as usize] + state.robots[rtype as usize] * t + (t * (t - 1) / 2)
}

fn simulate(blueprint: &Blueprint, time_left: u32) -> u32 {
    let state = State {
        robots: [1, 0, 0, 0],
        resources: [0, 0, 0, 0],
        time_left,
    };
    recurse(blueprint, &state, None, 0)
}

fn recurse(
    blueprint: &Blueprint,
    state: &State,
    skipped: Option<[bool; 4]>,
    curr_best: u32,
) -> u32 {
    // If we have at most one minute left, then we know how much geode we can produce.
    if state.time_left <= 1 {
        return state.resources[Type::Geode as usize] + state.robots[Type::Geode as usize];
    }

    // If the optimistic best of the current minute is worse than the current best,
    // then we can stop searching this branch.
    if optimistic_best(state, Type::Geode, None) < curr_best {
        return 0;
    }

    // If we have at least three minutes left, and our optimistic best for obsidian
    // production by two minutes remaining is not enough to purchase another geode
    // robot, then we know how much geode we can produce.
    if state.time_left >= 3
        && optimistic_best(state, Type::Obsidian, Some(state.time_left - 2))
            < blueprint.costs[Type::Geode as usize][2]
    {
        return state.resources[Type::Geode as usize]
            + state.time_left * state.robots[Type::Geode as usize];
    }

    let mut can_build = [false; 4];
    let mut best = curr_best;

    for rtype in [Type::Geode, Type::Obsidian, Type::Clay, Type::Ore] {
        if (skipped.is_none() || !skipped.unwrap()[rtype as usize])
            && state.can_build(blueprint, rtype)
        {
            can_build[rtype as usize] = true;
            let mut next_state = *state;
            next_state.gather();
            next_state.build(blueprint, rtype);
            next_state.time_left -= 1;
            let score = recurse(blueprint, &next_state, None, best);
            best = std::cmp::max(score, best);
        }
    }

    let mut next_state = *state;
    next_state.gather();
    next_state.time_left -= 1;
    let score = recurse(blueprint, &next_state, Some(can_build), best);
    std::cmp::max(score, best)
}

#[allow(unused_variables)]
pub fn part_one(input: &Vec<Blueprint>) -> Option<u32> {
    Some(
        input
            .par_iter()
            .map(|bp| bp.id * simulate(bp, 24))
            .sum::<u32>(),
    )
}

#[allow(unused_variables)]
pub fn part_two(input: &Vec<Blueprint>) -> Option<u32> {
    Some(
        input
            .par_iter()
            .take(3)
            .map(|bp| simulate(bp, 32))
            .product::<u32>(),
    )
}

fn main() {
    let input = advent_of_code::read_file("inputs", 19);
    let parsed = advent_of_code::run_parser(parse, &input);
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
        assert_eq!(part_one(&parsed), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        let parsed = parse(&input);
        assert_eq!(part_two(&parsed), Some(3472));
    }
}

use std::collections::HashMap;

const INPUT_STR: &str =include_str!("input/day_14.txt");

pub fn part_1() -> String {
    run_steps(10)
}

pub fn part_2() -> String {
    run_steps(40)
}


struct Reactor {
    rules: HashMap<(char, char), char>,
    cache: HashMap<(char, char, u64), HashMap<char, u64>>,
}

impl Reactor {
    fn new(rules: HashMap<(char, char), char>) -> Reactor {
        Reactor {
            rules,
            cache: HashMap::new()
        }
    }

    fn run_steps(&mut self, a: char, b: char, steps: u64) -> HashMap<char, u64> {
        if let Some(cache) = self.cache.get(&(a, b, steps)) {
            return cache.clone();
        }

        if steps == 0 {
            let mut stats = HashMap::new();
            stats.insert(b, 1);
            return stats
        }

        let c = *self.rules.get(&(a, b)).unwrap();
        let mut polymer_a = self.run_steps(a, c, steps - 1);
        let polymer_b = self.run_steps(c, b, steps - 1);

        for (chr, count) in polymer_b {
            let a_stats = *polymer_a.get(&chr).unwrap_or(&0);
            polymer_a.insert(chr, a_stats + count);
        }

        self.cache.insert((a, b, steps), polymer_a.clone());

        polymer_a
    }
}

fn run_steps(steps: u64) -> String {
    let (formula, rules) = get_input();
    let mut reactor = Reactor::new(rules);

    let mut p = formula[0];

    let mut stats = HashMap::new();
    stats.insert(p, 1);

    for char in formula.iter().skip(1) {
        let polymer = reactor.run_steps(p, *char, steps);
        for (chr, count) in polymer.iter() {
            let poly_next = *stats.get(&chr).unwrap_or(&0);
            stats.insert(*chr, poly_next + count);
        }
        p = *char;
    }

    let min = stats.iter().min_by_key(|(_, b)| **b).unwrap();
    let max = stats.iter().max_by_key(|(_, b)| **b).unwrap();

    format!("{:?}", max.1 - min.1)
}

fn get_input() -> (Vec<char>, HashMap<(char, char), char>) {
    let mut input = INPUT_STR.split("\n");

    let formula = input.next().unwrap().chars().collect();

    let mut rules = HashMap::new();

    for rule in input.skip(1) {
        let parts: Vec<_> = rule.chars().collect();
        rules.insert((parts[0], parts[1]), parts[6]);
    }

    (formula, rules)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::day_14::Reactor;
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_reactor() {
        let mut rules = HashMap::new();
        rules.insert(('A', 'B'), 'C');
        rules.insert(('B', 'A'), 'C');
        rules.insert(('A', 'C'), 'C');
        rules.insert(('C', 'A'), 'C');
        rules.insert(('B', 'C'), 'C');
        rules.insert(('C', 'B'), 'A');

        let mut reactor = Reactor::new(rules);

        let expected: HashMap<char, u64> = vec![('B', 1), ('C', 2), ('A', 1)]
            .into_iter().collect();
        assert_eq!(reactor.run_steps('A', 'B', 2), expected)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), "3095")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), "3152788426516")
    }
}

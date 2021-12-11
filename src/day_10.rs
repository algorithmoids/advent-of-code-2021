use std::collections::HashMap;

const INPUT_STR: &str = include_str!("input/day_10.txt");

pub fn part_1() -> String {
    let pairs: HashMap<_, _> = vec![
        ('[', ']'),
        ('(', ')'),
        ('{', '}'),
        ('<', '>'),
    ].into_iter().collect();

    let score_rule: HashMap<_, _> = vec![
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ].into_iter().collect();

    let mut score = 0;

    for row in get_input() {
        let mut brackets = vec![];

        for c in row.chars() {
            if pairs.contains_key(&c) {
                brackets.push(c);
            }
            else {
                match brackets.pop() {
                    Some(opening) => {
                        if pairs.get(&opening).unwrap() != &c {
                            score += score_rule.get(&c).unwrap();
                            break
                        }
                    },
                    None => break
                }
            }
        }
    }

    format!("{}", score)
}


pub fn part_2() -> String {
    let pairs: HashMap<_, _> = vec![
        ('[', ']'),
        ('(', ')'),
        ('{', '}'),
        ('<', '>'),
    ].into_iter().collect();

    let score_rule: HashMap<_, _> = vec![
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ].into_iter().collect();

    let mut scores = vec![];

    for row in get_input() {
        let mut brackets = vec![];

        let mut broken = false;

        for c in row.chars() {
            if pairs.contains_key(&c) {
                brackets.push(c);
            }
            else {
                match brackets.pop() {
                    Some(opening) => {
                        if pairs.get(&opening).unwrap() != &c {
                            brackets.push(opening);
                            broken = true;
                            break
                        }
                    },
                    None => break
                }
            }
        }

        if !broken {
            let mut score: i64 = 0;
            for c in brackets.iter().rev() {
                score = score * 5 + score_rule[c]
            }

            scores.push(score);
        }
    }

    scores.sort();

    format!("{}", scores[scores.len() / 2])
}


fn get_input() -> Vec<String> {
    INPUT_STR.split('\n')
        .map(|x| String::from(x))
        .collect()
}

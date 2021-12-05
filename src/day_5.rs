use regex::Regex;
use std::collections::HashMap;

const INPUT_STR: &str = include_str!("input/day_5.txt");

#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub fn part_1() -> String {
    let mut vent_map = HashMap::new();

    for vent in get_input() {
        let (x1, x2) = (vent.x1.min(vent.x2), vent.x1.max(vent.x2));
        let (y1, y2) = (vent.y1.min(vent.y2), vent.y1.max(vent.y2));

        if x1 == x2 || y1 == y2 {
            for x in x1 ..= x2 {
                for y in y1 ..= y2 {
                    vent_map.insert(
                        (x, y),
                        vent_map.get(&(x, y)).unwrap_or(&0) + 1
                    );
                }
            }
        }
    }

    let count = vent_map.values().into_iter().filter(|x| x != &&1).count();
    format!("{}", count)
}

pub fn part_2() -> String {
    let mut vent_map = HashMap::new();

    for vent in get_input() {
        let (x1, x2) = (vent.x1.min(vent.x2), vent.x1.max(vent.x2));
        let (y1, y2) = (vent.y1.min(vent.y2), vent.y1.max(vent.y2));

        if x1 == x2 || y1 == y2 {
            for x in x1 ..= x2 {
                for y in y1 ..= y2 {
                    vent_map.insert(
                        (x, y),
                        vent_map.get(&(x, y)).unwrap_or(&0) + 1
                    );
                }
            }
        }

        else if x2 - x1 == y2 - y1 {
            if vent.x2 - vent.x1 == vent.y2 - vent.y1 {
                for d in 0 ..= x2 - x1 {
                    vent_map.insert(
                        (x1 + d, y1 + d),
                        vent_map.get(&(x1 + d, y1 + d)).unwrap_or(&0) + 1
                    );
                }
            }
            else {
                for d in 0 ..= x2 - x1 {
                    vent_map.insert(
                        (x2 - d, y1 + d),
                        vent_map.get(&(x2 - d, y1 + d)).unwrap_or(&0) + 1
                    );
                }
            }
        }
    }

    let count = vent_map.values().into_iter().filter(|x| x != &&1).count();
    format!("{}", count)
}

fn get_input() -> Vec<Vent> {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    INPUT_STR.split('\n')
        .map(|x| {
            let captures: Vec<i32> =
                re.captures(x).unwrap()
                    .iter()
                    .skip(1)
                    .map(|x|x.unwrap().as_str().parse().unwrap())
                    .collect();

                Vent {
                    x1: captures[0].clone(),
                    y1: captures[1].clone(),
                    x2: captures[2].clone(),
                    y2: captures[3].clone(),
                }
        })
        .collect()
}

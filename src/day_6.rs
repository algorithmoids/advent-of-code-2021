use std::collections::HashMap;

const INPUT_STR: &str = include_str!("input/day_6.txt");

#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub fn part_1() -> String {
    count_all(80)
}

pub fn part_2() -> String {
    count_all(256)
}

fn count_all(days: i32) -> String {
    let mut total_fish = 0_u64;

    let mut fish = FishFarm::new();

    for count in get_input() {
        total_fish += 1 + fish.count_successors(days - count as i32 + 6)
    }

    format!("{}", total_fish)
}


struct FishFarm {
    cache: HashMap<i32, u64>
}

impl FishFarm {
    fn new() -> FishFarm {
        FishFarm {
            cache: HashMap::new()
        }
    }

    fn count_successors(&mut self, days: i32) -> u64 {
        if days < 7 {
            0
        } else {
            let mut remains = days - 9;
            let mut successors = (days / 7) as u64;
            while remains >= 7 {
                if self.cache.contains_key(&remains) {
                    successors += self.cache.get(&remains).unwrap();
                }
                else {
                    let day_successors = self.count_successors(remains);
                    self.cache.insert(remains, day_successors);
                    successors += day_successors;
                }
                remains -= 7;
            }
            return successors
        }
    }
}


fn get_input() -> Vec<u8> {
    INPUT_STR.split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

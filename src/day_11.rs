use std::collections::VecDeque;

const INPUT_STR: &str = include_str!("input/day_11.txt");

const NEIGBOURS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub fn part_1() -> String {
    let mut field = get_input();
    let mut flashes = 0;

    for _ in 0 .. 100 {
        flashes += run_step(&mut field);
    }

    format!("{:?}", flashes)
}

pub fn part_2() -> String {
    let mut field = get_input();

    let mut step = 0;
    loop {
        let flashes = run_step(&mut field);

        step += 1;

        if flashes == 100 {
            return format!("{:?}", step)
        }
    }
}

fn run_step(field: &mut Vec<Vec<u32>>) -> i32 {
    for i in 0..10 {
        for j in 0..10 {
            field[i][j] += 1;
        }
    }

    let mut lights = VecDeque::new();

    for i in 0 .. 10 {
        for j in 0 .. 10 {
            if field[i][j] == 10 {
                lights.push_front((i, j));
            }
        }
    }

    light_up(field, lights);

    let mut flashes = 0;

    for i in 0 .. 10 {
        for j in 0 .. 10 {
            if field[i][j] > 9 {
                flashes += 1;
                field[i][j] = 0;
            }
        }
    }

    flashes
}

fn light_up(field: &mut Vec<Vec<u32>>, mut lights: VecDeque<(usize, usize)>) {
    loop {
        match lights.pop_back() {
            Some((x, y)) => {
                for (dx, dy) in NEIGBOURS {
                    let (x, y) = (x as i32 + dx, y as i32 + dy);

                    if x >= 0 && x <= 9 && y >= 0 && y <= 9 {
                        let x = x as usize;
                        let y = y as usize;

                        field[x][y] += 1;
                        if field[x][y] == 10 {
                            lights.push_front((x, y));
                        }
                    }
                }
            }
            None => return
        }
    }
}

fn get_input() -> Vec<Vec<u32>> {
    INPUT_STR.split('\n')
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

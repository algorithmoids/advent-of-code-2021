use std::collections::HashMap;

const INPUT_STR: &str = include_str!("input/day_4.txt");


pub fn part_1() -> String {
    let (numbers, boards) = get_input();

    let mut map: HashMap<u8, Vec<(usize, usize, usize)>> = HashMap::new();

    for i in 0 .. boards.len() {
        for j in 0 .. 5 {
            for k in 0 .. 5 {
                let number = &boards[i][j][k];
                if !&map.contains_key(number) {
                    map.insert(*number, vec![]);
                }

                let matches = map.get_mut(&number).unwrap();
                matches.push((i, j, k))
            }
        }
    }

    let marker = vec![
        vec![false; 5]; 5
    ];

    let mut bingo: Vec<_> = (0 .. boards.len())
        .map(|_| marker.clone())
        .collect();

    for number in numbers {
        if map.contains_key(&number) {
            let positions = map.get(&number).unwrap();

            for (i, j, k) in positions {
                bingo[*i][*j][*k] = true;

                let mut vertical = true;
                let mut horizontal = true;

                for n in 0 .. 5 {
                    if !bingo[*i][*j][n]{
                        vertical = false;
                    }

                    if !bingo[*i][n][*k]{
                        horizontal = false;
                    }
                }

                if vertical || horizontal {
                    let mut unmarked = 0;

                    for board_i in 0 .. 5 {
                        for board_j in 0 .. 5 {
                            if !bingo[*i][board_i][board_j] {
                                unmarked += boards[*i][board_i][board_j] as i32;
                            }
                        }
                    }
                    return format!("{}", (number as i32) * unmarked)
                }

            }
        }
    }

    panic!("nobody won")
}

pub fn part_2() -> String {
    let (numbers, boards) = get_input();

    let mut map: HashMap<u8, Vec<(usize, usize, usize)>> = HashMap::new();

    for i in 0 .. boards.len() {
        for j in 0 .. 5 {
            for k in 0 .. 5 {
                let number = &boards[i][j][k];
                if !&map.contains_key(number) {
                    map.insert(*number, vec![]);
                }

                let matches = map.get_mut(&number).unwrap();
                matches.push((i, j, k))
            }
        }
    }

    let marker = vec![
        vec![false; 5]; 5
    ];

    let mut bingo: Vec<_> = (0 .. boards.len())
        .map(|_| marker.clone())
        .collect();

    let mut final_score = 0;
    let mut finished = vec![];

    for number in numbers {
        if map.contains_key(&number) {
            let positions = map.get(&number).unwrap();

            for (i, j, k) in positions {
                if !finished.contains(i) {
                    bingo[*i][*j][*k] = true;

                    let mut vertical = true;
                    let mut horizontal = true;

                    for n in 0..5 {
                        if !bingo[*i][*j][n] {
                            vertical = false;
                        }

                        if !bingo[*i][n][*k] {
                            horizontal = false;
                        }
                    }

                    if vertical || horizontal {
                        let mut unmarked = 0;

                        for board_i in 0 .. 5 {
                            for board_j in 0 .. 5 {
                                if !bingo[*i][board_i][board_j] {
                                    unmarked += boards[*i][board_i][board_j] as i32;
                                }
                            }
                        }
                        finished.push(i.clone());
                        final_score = (number as i32) * unmarked;
                    }
                }
            }
        }
    }

    format!("{}", final_score)
}

fn get_input() -> (Vec<u8>, Vec<Vec<Vec<u8>>>) {
    let lines: Vec<_> = INPUT_STR.split('\n').collect();

    let numbers: Vec<u8> = lines[0].split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut squares = vec![];
    let mut square = vec![];

    for line in &lines[2 ..] {
        if line == &"" {
            squares.push(square);
            square = vec![];
        }
        else {
            let row: Vec<u8> = line
                .split(" ")
                .filter(|x| x.len() != 0)
                .map(|x| x.parse().unwrap())
                .collect();

            square.push(row)
        }
    }

    squares.push(square);

    return (numbers, squares)
}


#[test]
fn test() {
    println!("{:?}", get_input());
}

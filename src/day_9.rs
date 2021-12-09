use std::collections::VecDeque;

const INPUT_STR: &str = include_str!("input/day_9.txt");

pub fn part_1() -> String {
    let field = get_input();

    let risk: u32 = get_minimums(&field)
        .into_iter()
        .map(|(x, y)| &field[x][y] + 1)
        .sum();

    format!("{}", risk)
}

pub fn part_2() -> String {
    let field = get_input();

    let mut marked = vec![vec![false; field[0].len()]; field.len()];

    let direction: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut sizes = vec![];

    for (x, y) in get_minimums(&field) {
        let mut size = 1;
        let mut queue = VecDeque::new();
        queue.push_front((x, y));

        while !queue.is_empty() {
            let (pi, pj) = queue.pop_back().unwrap();

            for (di, dj) in direction {
                let i = pi as i32 + di;
                let j = pj as i32 + dj;

                if i == -1 || i as usize == field[0].len() || j == -1 || j as usize == field.len() {
                    continue
                }

                let i = i as usize;
                let j = j as usize;

                if field[pi][pj] <= field[i][j] && !marked[i][j] && field[i][j] != 9 {
                    size += 1;
                    marked[i][j] = true;
                    queue.push_front((i, j))
                }
            }
        }

        sizes.push(size);
    }

    sizes.sort();
    let basias: i32 = sizes.iter().rev().take(3).product();

    format!("{}", basias)
}

fn get_minimums(field: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut mins = vec![];

    for i in 0 .. field.len() {
        for j in 0 .. field[0].len() {
            let mut min = true;
            if i != 0 && field[j][i-1] <= field[j][i] {
                min = false
            }
            if i != field.len() - 1 && field[j][i+1] <= field[j][i] {
                min = false
            }
            if j != 0 && field[j-1][i] <= field[j][i] {
                min = false
            }
            if j != field[0].len() - 1 && field[j+1][i] <= field[j][i] {
                min = false
            }

            if min {
                mins.push((j as usize, i as usize))
            }
        }
    }

    mins
}

fn get_input() -> Vec<Vec<u32>> {
    INPUT_STR.split('\n')
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

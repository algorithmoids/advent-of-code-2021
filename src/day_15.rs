use std::collections::VecDeque;

const INPUT_STR: &str =include_str!("input/day_15.txt");

pub fn part_1() -> String {
    let input = get_input();
    find_risk(input)
}

pub fn part_2() -> String {
    let input = get_input();
    let w = input[0].len();
    let h = input.len();

    let mut big_input = vec![vec![0_u8; w * 5]; h * 5];
    for i in 0 .. h * 5 {
        for j in 0 .. w * 5 {
            let value = input[i % h][j % w];
            let value = (value + (i / h) as u8 + (j / w) as u8 - 1) % 9 + 1;
            big_input[i][j] = value
        }
    }

    find_risk(big_input)
}

pub fn find_risk(input: Vec<Vec<u8>>) -> String {
    let w = input[0].len();
    let h = input.len();

    let mut risk = vec![vec![0_u32; w]; h];
    let mut queue = VecDeque::new();
    queue.push_front((0, 0, 0_u32, (0, 0, 0_u32)));

    while !queue.is_empty() {
        // print_risk(&risk);
        let (x, y, path_risk, old) = queue.pop_back().unwrap();
        if (risk[x][y] > path_risk || risk[x][y] == 0) && risk[old.0][old.1] == old.2 {
            risk[x][y] = path_risk;
            if x > 0 {queue.push_front((x-1, y, path_risk + input[x-1][y] as u32, (x, y, path_risk))) }
            if x < h -1 {queue.push_front((x+1, y, path_risk + input[x+1][y] as u32, (x, y, path_risk))) }
            if y > 0 {queue.push_front((x, y-1, path_risk + input[x][y-1] as u32, (x, y, path_risk))) }
            if y < w - 1 {queue.push_front((x, y+1, path_risk + input[x][y+1] as u32, (x, y, path_risk))) }
        }
    }

    format!("{:?}", risk[h-1][w-1])
}

fn get_input() -> Vec<Vec<u8>> {
    INPUT_STR.split("\n").into_iter()
        .map(|x| x.chars()
            .into_iter()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), "595")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), "2914")
    }
}

const INPUT_STR: &str = include_str!("input/day_3.txt");


pub fn part_1() -> String {
    let input = get_input();
    let n_bits = input[0].len();

    let total_count = input.len();
    let mut ones = vec![0_usize; n_bits];

    for row in input {
        for i in 0 .. n_bits {
            let inc: usize = row[i].into();
            ones[i] += inc;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in ones {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if usize::from(bit) > total_count / 2 {
            gamma += 1
        }
        else {
            epsilon += 1
        }
    }

    format!("{:?}", gamma * epsilon)
}


pub fn part_2() -> String {
    let input = get_input();
    let oxygen_row = get_most_popular(input.clone(), 0, highest);

    let mut oxygen = 0_u32;
    for i in 0 .. oxygen_row.len() {
        oxygen += (oxygen_row[i] as u32) << ((oxygen_row.len()- i) as u32 - 1);
    }

    let co2_row = get_most_popular(input, 0, lowest);

    let mut co2 = 0_u32;
    for i in 0 .. co2_row.len() {
        co2 += (co2_row[i] as u32) << ((co2_row.len()- i) as u32 - 1);
    }

    format!("{:?}", oxygen * co2)
}


fn get_most_popular<F>(rows: Vec<Vec<u8>>, digit: usize, cmp: F) -> Vec<u8>
        where F: Fn(Vec<Vec<u8>>, Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    if rows.len() == 1 {
        return rows[0].clone();
    }

    let mut zeroes = vec![];
    let mut ones = vec![];

    for row in rows {
        if row[digit] == 1 {
            ones.push(row);
        }
        else {
            zeroes.push(row);
        }
    }

    let rows = cmp(ones, zeroes);
    get_most_popular(rows, digit + 1, cmp)
}

fn highest<T>(ones: Vec<T>, zeroes: Vec<T>) -> Vec<T> {
    if ones.len() >= zeroes.len() {
        ones
    }
    else {
        zeroes
    }
}

fn lowest<T>(ones: Vec<T>, zeroes: Vec<T>) -> Vec<T> {
    if zeroes.len() <= ones.len() {
        zeroes
    }
    else {
        ones
    }
}


fn get_input() -> Vec<Vec<u8>> {
    INPUT_STR.split('\n')
        .map(|x| x
            .as_bytes()
            .into_iter()
            .map(|y| y - 48)
            .collect())
        .collect()
}


#[test]
fn test() {
    println!("{:?}", get_input());
}
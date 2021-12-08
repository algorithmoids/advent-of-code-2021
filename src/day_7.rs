
const INPUT_STR: &str = include_str!("input/day_7.txt");

pub fn part_1() -> String {
    let mut crabs = get_input();
    crabs.sort();

    let start = crabs[0];
    let finish = *crabs.last().unwrap();

    let mut down = crabs.iter().sum::<u32>() as u32 + crabs.len() as u32;
    let mut up = 0_u32;

    let mut min_change = down;

    let mut on_line = 0;

    for i in start ..= finish {
        while crabs[on_line] < i {
            on_line += 1;
        }

        up += on_line as u32;
        down -= crabs.len() as u32 - on_line as u32;

        min_change = min_change.min(down + up);
    }

    format!("{:?}", min_change)
}

pub fn part_2() -> String {
    let mut crabs = get_input();
    crabs.sort();

    let start = crabs[0];
    let finish = *crabs.last().unwrap();

    let mut on_line = 0;

    let mut ups = vec![];
    let mut ups_inc = 0;
    let mut last_inc = 0;

    for i in start ..= finish {
        while crabs[on_line] < i {
            on_line += 1;
        }

        ups_inc += on_line;
        last_inc += ups_inc;
        ups.push(last_inc);
    }

    let mut downs = vec![];
    let mut downs_inc = 0;
    let mut last_down_inc = 0;
    let mut on_line = crabs.len() - 1;

    for i in start ..= finish {
        while crabs[on_line] > finish - start - i {
            on_line -= 1;
        }

        downs_inc += crabs.len() - 1 - on_line;
        last_down_inc += downs_inc;
        downs.insert(0, last_down_inc);
    }

    format!("{:?}", ups.iter().zip(downs).map(|(a, b)| a + b).min().unwrap())
}

fn get_input() -> Vec<u32> {
    INPUT_STR.split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

const INPUT_STR: &str = include_str!("input/day_2.txt");


pub fn part1() {
    let input = get_input();

    let mut length = 0;
    let mut depth = 0;

    for (direction, distance) in input {
        match direction.as_str() {
            "forward" => length += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => panic!("Unexpected command")
        }
    }

    println!("{:?}", length * depth);
}


pub fn part2() {
    let input = get_input();

    let mut length = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (direction, distance) in input {
        match direction.as_str() {
            "forward" => {
                length += distance;
                depth += distance * aim;
            },
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => panic!("Unexpected command")
        }
    }

    println!("{:?}", length * depth);
}


fn get_input() -> Vec<(String, i32)> {
    INPUT_STR.split('\n')
        .map(|x| {
            let command = x.split(' ').collect::<Vec<_>>();
            if let &[direction, distance] = command.as_slice() {
                Some((String::from(direction), distance.parse::<i32>().unwrap()))
            }
            else {
                None
            }
        })
        .filter(|x|x.is_some())
        .map(|x|x.unwrap())
        .collect::<Vec<_>>()
}


#[test]
fn test() {
    println!("{:?}", get_input());
}
const INPUT: &str = include_str!("input/day_1.txt");

fn part1() {

    let numbers: Vec<i32> = INPUT
        .split("\n")
        .map(|x| x.parse())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();

    let mut increasing = 0;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] {
            increasing += 1
        }
    }

    println!("{}", increasing)
}


fn part2() {

    let numbers: Vec<i32> = INPUT
        .split("\n")
        .map(|x| x.parse())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();

    let mut groups = vec![];

    for i in 0..numbers.len() - 2 {
        groups.push(numbers[i] + numbers[i+1] + numbers[i+2])
    }

    let mut increasing = 0;

    for i in 1..groups.len() {
        if groups[i] > groups[i-1] {
            increasing += 1
        }
    }

    println!("{}", increasing)
}
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

const INPUT_STR: &str = include_str!("input/day_8.txt");

pub fn part_1() -> String {
    let count = get_input()
        .iter()
        .map(|(_, signal)| signal)
        .flatten()
        .filter(|x| vec![2, 3, 4, 7].contains(&x.len()))
        .count();

    format!("{}", count)
}

pub fn part_2() -> String {
    let mut sum = 0;

    for (digits, signal) in get_input() {
        let digit_sets = &digits.iter().map(|x| HashSet::<&u8>::from_iter(x.as_bytes())).collect::<Vec<_>>();

        let mut numbers: HashMap<Vec<_>, u8> = HashMap::new();

        let one = digit_sets.iter().filter(|x| x.len() == 2).collect::<Vec<_>>()[0];
        let seven = digit_sets.iter().filter(|x| x.len() == 3).collect::<Vec<_>>()[0];
        let four = digit_sets.iter().filter(|x| x.len() == 4).collect::<Vec<_>>()[0];
        let eight = digit_sets.iter().filter(|x| x.len() == 7).collect::<Vec<_>>()[0];

        let mut one_list: Vec<&&u8> = one.iter().collect();
        one_list.sort();
        numbers.insert(one_list, 1);

        let mut seven_list: Vec<&&u8> = seven.iter().collect();
        seven_list.sort();
        numbers.insert(seven_list, 7);

        let mut four_list: Vec<&&u8> = four.iter().collect();
        four_list.sort();
        numbers.insert(four_list, 4);

        let mut eight_list: Vec<&&u8> = eight.iter().collect();
        eight_list.sort();
        numbers.insert(eight_list, 8);

        for digit in digit_sets {
            let mut digit_vec: Vec<&&u8> = digit.iter().collect();
            digit_vec.sort();

            if !vec![2, 3, 4, 7].contains(&digit.len()) {
                if digit.len() == 5 && (one - digit).is_empty() {
                    numbers.insert(digit_vec, 3);
                }
                else if digit.len() == 5 && (digit - four).len() == 3 {
                    numbers.insert(digit_vec, 2);
                }
                else if digit.len() == 5 && (digit - four).len() == 2 {
                    numbers.insert(digit_vec, 5);
                }
                else if digit.len() == 6 && (one - digit).len() == 1 {
                    numbers.insert(digit_vec, 6);
                }
                else if digit.len() == 6 && (four - digit).is_empty() {
                    numbers.insert(digit_vec, 9);
                }
                else {
                    numbers.insert(digit_vec, 0);
                }
            }
        }

        let mut signal_value: u32 = 0;

        for digit in signal {
            let digit: Vec<&u8> = digit.as_bytes().iter().collect::<Vec<_>>();
            let mut digit2: Vec<_> = digit.iter().collect();
            digit2.sort();

            signal_value = signal_value * 10 + (*numbers.get(&digit2).unwrap()) as u32;
        }

        sum += signal_value;
    }

    format!("{}", sum)
}

fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    let mut numbers = vec![];
    for row in INPUT_STR.split('\n') {
        let x: Vec<String> = row.split(" | ").map(|x| String::from(x)).collect();
        numbers.push((
            x[0].split(" ").map(|x| String::from(x)).collect(),
            x[1].split(" ").map(|x| String::from(x)).collect())
        )
    }
    numbers
}

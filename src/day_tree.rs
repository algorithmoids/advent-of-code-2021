use std::borrow::BorrowMut;

const INPUT_STR: &str = include_str!("input/day_3.txt");

struct Digit {
    count: usize,
    zero: Option<Box<Digit>>,
    one: Option<Box<Digit>>,
}

impl Digit {
    fn new() -> Digit {
        Digit {
            count: 0,
            zero: None,
            one: None,
        }
    }
}


pub fn part_2() -> String {
    let input = get_input();

    let mut tree = Digit::new();

    for row in input {
        let mut digit = tree.borrow_mut();

        for d in row {
            if d == 0 {
                if digit.zero.is_none() {
                    digit.zero = Some(Box::new(Digit::new()))
                }
                digit = digit.zero.as_mut().unwrap().as_mut();

            }
            if d == 1 {
                if digit.one.is_none() {
                    digit.one = Some(Box::new(Digit::new()))
                }
                digit = digit.one.as_mut().unwrap().as_mut();
            }

            digit.count += 1;
        }
    }

    let oxygen = collect_tree(&tree, 0, highest);
    let co2 = collect_tree(&tree, 0, lowest);

    format!("{}", oxygen * co2)
}

fn collect_tree<'a, F>(tree: &'a Digit, number: i32, cmp: F) -> i32
        where F: Fn(&'a Digit, &'a Digit) -> (&'a Digit, i32) {

    match (&tree.zero, &tree.one) {
        (Some(zero), Some(one)) => {
            let (tree, inc) = cmp(one, zero);
            collect_tree(tree, (number << 1) + inc, cmp)
        },
        (None, Some(tree)) => collect_tree(tree, (number << 1) + 1, cmp),
        (Some(tree), None) => collect_tree(tree, number << 1, cmp),
        (None, None) => return number
    }
}

fn highest<'a>(ones: &'a Digit, zeroes: &'a Digit) -> (&'a Digit, i32) {
    if ones.count >= zeroes.count {
        (ones, 1)
    }
    else {
        (zeroes, 0)
    }
}

fn lowest<'a>(ones: &'a Digit, zeroes: &'a Digit) -> (&'a Digit, i32) {
    if zeroes.count <= ones.count {
        (zeroes, 0)
    }
    else {
        (ones, 1)
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
    assert_eq!(part_2(), "2555739");
}
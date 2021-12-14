use std::collections::HashSet;

const INPUT_STR: &str = include_str!("input/day_13.txt");


pub fn part_1() -> String {
    let (points, folds) = get_input();
    let (axis, line) = &folds[0];

    let folded = fold(points, axis, line);

    format!("{:?}", folded.len())
}


pub fn part_2() -> String {
    let (mut points, folds) = get_input();

    for (axis, line) in folds {
        points = fold(points, &axis, &line);
    }

    make_paper(points)
}


fn fold(points: Vec<(i32, i32)>, axis: &String, line: &i32) -> Vec<(i32, i32)> {
    let mut folded = HashSet::new();

    for (i, j) in points {
        if axis.as_str() == "x" {
            if i < *line {
                folded.insert((i, j));
            } else {
                folded.insert((2 * line - i, j));
            }
        } else {
            if j < *line {
                folded.insert((i, j));
            } else {
                folded.insert((i, 2 * line - j));
            }
        }
    }

    folded.into_iter().collect()
}


fn make_paper(points: Vec<(i32, i32)>) -> String {
    let w = points.iter().map(|(x, _)| x).max().unwrap();
    let h = points.iter().map(|(_, y)| y).max().unwrap();

    let mut paper = String::new();

    for j in 0 ..= *h {
        for i in 0 ..= *w {
            if points.contains(&(i, j)) {
                paper += "##";
            }
            else {
                paper += "..";
            }
        }
        paper += "\n";
    }

    paper
}


fn get_input() -> (Vec<(i32, i32)>, Vec<(String, i32)>) {
    let input: Vec<_> = INPUT_STR.split("\n\n").collect();

    let mut points: Vec<(i32, i32)> = vec![];

    for row in input[0].split('\n') {
        let point_parts: Vec<_> = row.split(',').collect();
        points.push((
            point_parts[0].parse().unwrap(),
            point_parts[1].parse().unwrap()
        ));
    }

    let mut folds = vec![];

    for row in input[1].split('\n') {
        let point_parts: Vec<_> = row.split(' ').last().unwrap().split('=').collect();

        folds.push((
            String::from(point_parts[0]),
            point_parts[1].parse().unwrap()
        ));
    }

    (points, folds)
}

#[cfg(test)]
mod test {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), "706")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), r#"
##........######....########......####..######........####..########..##....##
##........##....##..##..............##..##....##........##..##........##....##
##........##....##..######..........##..######..........##..######....########
##........######....##..............##..##....##........##..##........##....##
##........##..##....##........##....##..##....##..##....##..##........##....##
########..##....##..##..........####....######......####....########..##....##
"#.trim_start())
    }
}

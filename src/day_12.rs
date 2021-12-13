use std::collections::HashMap;

const INPUT_STR: &str = include_str!("input/day_12.txt");


pub fn part_1() -> String {
    let graph = get_graph();
    let routes = make_routes(&graph, vec![String::from("start")]);

    format!("{:?}", routes)
}

fn make_routes(graph: &HashMap<String, Vec<String>>, route: Vec<String>) -> i32 {
    let mut routes = 0;
    let cave = route.last().unwrap();

    for next_step in graph.get(cave).unwrap() {
        if next_step == &String::from("end") {
            routes += 1;
        }
        else if &next_step.to_uppercase() == next_step || !route.contains(next_step) {
            let mut next_route = route.clone();
            next_route.push(String::from(next_step));

            routes += make_routes(graph, next_route);
        }
    }

    routes
}

pub fn part_2() -> String {
    let graph = get_graph();
    let routes = make_routes_2(&graph, vec![String::from("start")], false);

    format!("{:?}", routes)
}

fn make_routes_2(graph: &HashMap<String, Vec<String>>, route: Vec<String>, been_twice: bool) -> i32 {
    let mut routes = 0;
    let cave = route.last().unwrap();

    for next_step in graph.get(cave).unwrap() {
        // println!("{:?} {}", route, been_twice);
        if next_step == &String::from("end") {
            routes += 1;
        }
        else if &next_step.to_uppercase() == next_step {
            let mut next_route = route.clone();
            next_route.push(String::from(next_step));

            routes += make_routes_2(graph, next_route, been_twice);
        }
        else if next_step != &String::from("end") && next_step != &String::from("start") &&
            (!route.contains(next_step) || !been_twice)
        {
            // println!("{:?} {} {}", route, next_step, been_twice);
            let mut next_route = route.clone();
            next_route.push(String::from(next_step));

            let twice = been_twice || route.contains(next_step);
            routes += make_routes_2(graph, next_route, twice);
        }
    }

    routes
}


fn get_graph() -> HashMap<String, Vec<String>> {
    let edges = get_input();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (a, b) in edges {
        if !graph.contains_key(&a) {
            graph.insert(a.clone(), vec![]);
        }
        graph.get_mut(&a).unwrap().push(b.clone());

        if !graph.contains_key(&b) {
            graph.insert(b.clone(), vec![]);
        }
        graph.get_mut(&b).unwrap().push(a)
    }

    graph
}

fn get_input() -> Vec<(String, String)> {
    let mut edges = vec![];

    for row in INPUT_STR.split('\n') {
        let points: Vec<_> = row.split('-').collect();
        edges.push((String::from(points[0]), String::from(points[1])));
    }

    edges
}

#[test]
fn test() {
    println!("{:?}", part_2())
}

#[test]
fn test2() {
    println!("{:?}", &String::from("abcd") == &String::from("abcd"))
}

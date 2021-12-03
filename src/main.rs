use std::time::Instant;

mod day_1;
mod day_2;
mod day_3;
mod day_tree;


fn main() {
    print_time("Day 1.1", day_1::part_1);
    print_time("Day 1.1", day_1::part_2);
    print_time("Day 2.1", day_2::part_1);
    print_time("Day 2.2", day_2::part_2);
    print_time("Day 3.1", day_3::part_1);
    print_time("Day 3.2", day_3::part_2);
    print_time("Day 3.2 (tree)", day_tree::part_2);
}


fn print_time<F>(title: &str, task: F)
        where F: Fn() -> String {
    let now = Instant::now();
    let result = task();
    let elapsed = now.elapsed().as_secs_f64();
    println!("{}: {} ({} s)", title, result, elapsed)
}
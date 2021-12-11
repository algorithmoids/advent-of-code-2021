use std::time::Instant;

mod day_1;
mod day_2;
mod day_3;
mod day_tree;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;


fn main() {
    print_time("Day 1.1", day_1::part_1);
    print_time("Day 1.1", day_1::part_2);
    print_time("Day 2.1", day_2::part_1);
    print_time("Day 2.2", day_2::part_2);
    print_time("Day 3.1", day_3::part_1);
    print_time("Day 3.2", day_3::part_2);
    print_time("Day 3.2 (tree)", day_tree::part_2);
    print_time("Day 4.1", day_4::part_1);
    print_time("Day 4.2", day_4::part_2);
    print_time("Day 5.1", day_5::part_1);
    print_time("Day 5.2", day_5::part_2);
    print_time("Day 6.1", day_6::part_1);
    print_time("Day 6.2", day_6::part_2);
    print_time("Day 7.1", day_7::part_1);
    print_time("Day 7.2", day_7::part_2);
    print_time("Day 8.1", day_8::part_1);
    print_time("Day 8.2", day_8::part_2);
    print_time("Day 9.1", day_9::part_1);
    print_time("Day 9.2", day_9::part_2);
    print_time("Day 10.1", day_10::part_1);
    print_time("Day 10.2", day_10::part_2);
    print_time("Day 11.1", day_11::part_1);
    print_time("Day 11.2", day_11::part_2);
}


fn print_time<F>(title: &str, task: F)
        where F: Fn() -> String {
    let now = Instant::now();
    let result = task();
    let elapsed = now.elapsed().as_secs_f64();
    println!("{}: {} | {:.6} s", title, result, elapsed)
}

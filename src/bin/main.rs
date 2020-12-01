use aoc2020::{self, read_data};

fn main() {
    print!("\nAdvent of Code 2020\n--------------------\n\n");

    // Day 1
    if let Ok(data_day_1) = read_data("data/day1.txt") {
        let day_1_task_1 = aoc2020::day_1(&data_day_1, 2, 2020);
        println!("AOC 2020, day 1, task 1: {}", day_1_task_1);
        assert_eq!(day_1_task_1, 270144);

        let day_1_task_2 = aoc2020::day_1(&data_day_1, 3, 2020);
        println!("AOC 2020, day 1, task 2: {}", day_1_task_2);
        assert_eq!(day_1_task_2, 261342720);
    } else {
        eprintln!("AOC 2020, day 1: Unable to read data for day 1. Skipping.");
    }
}

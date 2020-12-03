use aoc2020::{self, read_data};

fn main() {
    print!("\nAdvent of Code 2020\n--------------------\n\n");

    // Day 1
    if let Ok(data_day_1) = read_data::<u32, _>("data/day1.txt") {
        let day_1_task_1 = aoc2020::day_1::day_1(&data_day_1, 2, 2020);
        println!("AOC 2020, day 1, task 1: {}", day_1_task_1);
        assert_eq!(day_1_task_1, 270144);

        let day_1_task_2 = aoc2020::day_1::day_1(&data_day_1, 3, 2020);
        println!("AOC 2020, day 1, task 2: {}", day_1_task_2);
        assert_eq!(day_1_task_2, 261342720);
    } else {
        eprintln!("AOC 2020, day 1: Unable to read data for day 1. Skipping.");
    }

    if let Ok(data_day_2) = read_data::<String, _>("data/day2.txt") {
        let day_2_task_1 = aoc2020::day_2::task_1(&data_day_2);
        println!("AOC 2020, day 2, task 1: {}", day_2_task_1);
        assert_eq!(day_2_task_1, 528);

        let day_2_task_2 = aoc2020::day_2::task_2(&data_day_2);
        println!("AOC 2020, day 2, task 2: {}", day_2_task_2);
        assert_eq!(day_2_task_2, 497);
    } else {
        eprintln!("AOC 2020, day 2: Unable to read data for day 2. Skipping.");
    }

    if let Ok(data_day_3) = read_data::<String, _>("data/day3.txt") {
        let day_3_task_1 = aoc2020::day_3::task_1(&data_day_3, 3, 1);
        println!("AOC 2020, day 3, task 1: {}", day_3_task_1);
        assert_eq!(day_3_task_1, 156);

        let day_3_task_2 =
            aoc2020::day_3::task_2(&data_day_3, &vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
        println!("AOC 2020, day 3, task 2: {}", day_3_task_2);
        assert_eq!(day_3_task_2, 3521829480);
    } else {
        eprintln!("AOC 2020, day 3: Unable to read data for day 3. Skipping.");
    }
}

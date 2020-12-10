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

    // Day 2
    if let Ok(data_day_2) = read_data::<String, _>("data/day2.txt") {
        let day_2_task_1 = aoc2020::day_2::task_1(&data_day_2);
        println!("\nAOC 2020, day 2, task 1: {}", day_2_task_1);
        assert_eq!(day_2_task_1, 528);

        let day_2_task_2 = aoc2020::day_2::task_2(&data_day_2);
        println!("AOC 2020, day 2, task 2: {}", day_2_task_2);
        assert_eq!(day_2_task_2, 497);
    } else {
        eprintln!("AOC 2020, day 2: Unable to read data for day 2. Skipping.");
    }

    // Day 3
    if let Ok(data_day_3) = read_data::<String, _>("data/day3.txt") {
        let day_3_task_1 = aoc2020::day_3::task_1(&data_day_3, 3, 1);
        println!("\nAOC 2020, day 3, task 1: {}", day_3_task_1);
        assert_eq!(day_3_task_1, 156);

        let day_3_task_2 =
            aoc2020::day_3::task_2(&data_day_3, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
        println!("AOC 2020, day 3, task 2: {}", day_3_task_2);
        assert_eq!(day_3_task_2, 3521829480);
    } else {
        eprintln!("AOC 2020, day 3: Unable to read data for day 3. Skipping.");
    }

    // Day 4
    match std::fs::read_to_string("data/day4.txt") {
        Ok(content) => {
            let day_4_task_1 = aoc2020::day_4::task_1(&content);
            println!("\nAOC 2020, day 4, task 1: {}", day_4_task_1);
            assert_eq!(day_4_task_1, 222);

            let day_4_task_2 = aoc2020::day_4::task_2(&content);
            println!("AOC 2020, day 4, task 2: {}", day_4_task_2);
            assert_eq!(day_4_task_2, 140);
        }
        Err(_) => eprintln!("AOC 2020, day 4: Unable to read data for day 4. Skipping"),
    }

    // Day 5
    if let Ok(data_day_5) = read_data::<String, _>("data/day5.txt") {
        let day_5_task_1 = aoc2020::day_5::task_1(&data_day_5);
        println!("\nAOC 2020, day 5, task 1: {}", day_5_task_1);
        assert_eq!(day_5_task_1, 935);

        let day_5_task_2 = aoc2020::day_5::task_2(&data_day_5);
        println!("AOC 2020, day 5, task 2: {}", day_5_task_2);
        assert_eq!(day_5_task_2, 743);
    } else {
        eprintln!("AOC 2020, day 5: Unable to read data for day 5. Skipping.");
    }

    // Day 6
    match std::fs::read_to_string("data/day6.txt") {
        Ok(content) => {
            let day_6_task_1 = aoc2020::day_6::task_1(&content);
            println!("\nAOC 2020, day 6, task 1: {}", day_6_task_1);
            assert_eq!(day_6_task_1, 6542);

            let day_6_task_2 = aoc2020::day_6::task_2(&content);
            println!("AOC 2020, day 6, task 2: {}", day_6_task_2);
            assert_eq!(day_6_task_2, 3299);
        }
        Err(_) => eprintln!("AOC 2020, day 6: Unable to read data for day 6. Skipping"),
    }

    // Day 7
    if let Ok(data_day_7) = read_data::<String, _>("data/day7.txt") {
        let day_7_task_1 = aoc2020::day_7::task_1(&data_day_7);
        println!("\nAOC 2020, day 7, task 1: {}", day_7_task_1);
        assert_eq!(day_7_task_1, 261);

        let day_7_task_2 = aoc2020::day_7::task_2(&data_day_7);
        println!("AOC 2020, day 7, task 2: {}", day_7_task_2);
    // assert_eq!(day_7_task_2, 3335);
    } else {
        eprintln!("AOC 2020, day 7: Unable to read data for day 7. Skipping.");
    }

    // Day 8
    if let Ok(data_day_8) = read_data::<String, _>("data/day8.txt") {
        let day_8_task_1 = aoc2020::day_8::task_1(&data_day_8);
        println!("\nAOC 2020, day 8, task 1: {}", day_8_task_1);
        assert_eq!(day_8_task_1, 2080);

        let day_8_task_2 = aoc2020::day_8::task_2(&data_day_8);
        println!("AOC 2020, day 8, task 2: {}", day_8_task_2);
        assert_eq!(day_8_task_2, 2477);
    } else {
        eprintln!("AOC 2020, day 8: Unable to read data for day 8. Skipping.");
    }

    // Day 9
    if let Ok(data_day_9) = read_data::<u64, _>("data/day9.txt") {
        let day_9_task_1 = aoc2020::day_9::task_1(&data_day_9, 25);
        println!("\nAOC 2020, day 9, task 1: {}", day_9_task_1);
        assert_eq!(day_9_task_1, 257342611);

        let day_9_task_2 = aoc2020::day_9::task_2(&data_day_9, 25);
        println!("AOC 2020, day 9, task 2: {}", day_9_task_2);
        assert_eq!(day_9_task_2, 35602097);
    } else {
        eprintln!("AOC 2020, day 9: Unable to read data for day 9. Skipping.");
    }
}

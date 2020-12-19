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
        assert_eq!(day_7_task_2, 3765);
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

    // Day 10
    if let Ok(data_day_10) = read_data::<u32, _>("data/day10.txt") {
        let day_10_task_1 = aoc2020::day_10::task_1(&data_day_10);
        println!("\nAOC 2020, day 10, task 1: {}", day_10_task_1);
        assert_eq!(day_10_task_1, 2059);

        let day_10_task_2 = aoc2020::day_10::task_2(&data_day_10);
        println!("AOC 2020, day 10, task 2: {}", day_10_task_2);
        assert_eq!(day_10_task_2, 86812553324672);
    } else {
        eprintln!("AOC 2020, day 10: Unable to read data for day 10. Skipping.");
    }

    // Day 11
    match std::fs::read_to_string("data/day11.txt") {
        Ok(content) => {
            let day_11_task_1 =
                aoc2020::day_11::task_1_2(&content, &aoc2020::day_11::Strategy::Adjacent, 4);
            println!("\nAOC 2020, day 11, task 1: {}", day_11_task_1);
            assert_eq!(day_11_task_1, 2361);

            let day_11_task_2 =
                aoc2020::day_11::task_1_2(&content, &aoc2020::day_11::Strategy::Visible, 5);
            println!("AOC 2020, day 11, task 2: {}", day_11_task_2);
            assert_eq!(day_11_task_2, 2119);
        }
        Err(_) => eprintln!("AOC 2020, day 11: Unable to read data for day 11. Skipping."),
    }

    // Day 12
    if let Ok(data_day_12) = read_data::<String, _>("data/day12.txt") {
        let day_12_task_1 = aoc2020::day_12::task_1(&data_day_12);
        println!("\nAOC 2020, day 12, task 1: {}", day_12_task_1);
        assert_eq!(day_12_task_1, 1482);

        let day_12_task_2 = aoc2020::day_12::task_2(&data_day_12);
        println!("AOC 2020, day 12, task 2: {}", day_12_task_2);
        assert_eq!(day_12_task_2, 48739);
    } else {
        eprintln!("AOC 2020, day 8: Unable to read data for day 8. Skipping.");
    }

    // Day 13
    match std::fs::read_to_string("data/day13.txt") {
        Ok(content) => {
            let day_13_task_1 = aoc2020::day_13::task_1(&content);
            println!("\nAOC 2020, day 13, task 1: {}", day_13_task_1);
            assert_eq!(day_13_task_1, 5257);

            let day_13_task_2 = aoc2020::day_13::task_2(&content);
            println!("AOC 2020, day 13, task 2: {}", day_13_task_2);
            assert_eq!(day_13_task_2, 538703333547789);
        }
        Err(_) => eprintln!("AOC 2020, day 13: Unable to read data for day 13. Skipping."),
    }

    // Day 14
    if let Ok(data_day_14) = read_data::<String, _>("data/day14.txt") {
        let day_14_task_1 = aoc2020::day_14::task_1(&data_day_14);
        println!("\nAOC 2020, day 14, task 1: {}", day_14_task_1);
        assert_eq!(day_14_task_1, 11884151942312);

        let day_14_task_2 = aoc2020::day_14::task_2(&data_day_14);
        println!("AOC 2020, day 14, task 2: {}", day_14_task_2);
        assert_eq!(day_14_task_2, 2625449018811);
    } else {
        eprintln!("AOC 2020, day 14: Unable to read data for day 14. Skipping.");
    }

    // Day 15
    let data_day_15 = [18, 11, 9, 0, 5, 1];

    let day_15_task_1 = aoc2020::day_15::task_1_2(&data_day_15, 2020);
    println!("\nAOC 2020, day 15, task 1: {}", day_15_task_1);
    assert_eq!(day_15_task_1, 959);

    let day_15_task_2 = aoc2020::day_15::task_1_2(&data_day_15, 30000000);
    println!("AOC 2020, day 15, task 2: {}", day_15_task_2);
    assert_eq!(day_15_task_2, 116590);

    // Day 16
    if let Ok(data_day_16) = read_data::<String, _>("data/day16.txt") {
        let day_16_task_1 = aoc2020::day_16::task_1(&data_day_16);
        println!("\nAOC 2020, day 16, task 1: {}", day_16_task_1);
        assert_eq!(day_16_task_1, 23954);

        let day_16_task_2 = aoc2020::day_16::task_2(&data_day_16);
        println!("AOC 2020, day 16, task 2: {}", day_16_task_2);
        assert_eq!(day_16_task_2, 453459307723);
    } else {
        eprintln!("AOC 2020, day 16: Unable to read data for day 16. Skipping.");
    }

    // Day 17
    if let Ok(data_day_17) = std::fs::read_to_string("data/day17.txt") {
        let day_17_task_1 = aoc2020::day_17::task_1(&data_day_17, 6);
        println!("\nAOC 2020, day 17, task 1: {}", day_17_task_1);
        assert_eq!(day_17_task_1, 252);

        let day_17_task_2 = aoc2020::day_17::task_2(&data_day_17, 6);
        println!("AOC 2020, day 17, task 2: {}", day_17_task_2);
        assert_eq!(day_17_task_2, 2160);
    } else {
        eprintln!("AOC 2020, day 17: Unable to read data for day 17. Skipping.");
    }

    // Day 18
    if let Ok(data_day_18) = read_data::<String, _>("data/day18.txt") {
        let day_18_task_1 = aoc2020::day_18::task_1(&data_day_18);
        println!("\nAOC 2020, day 18, task 1: {}", day_18_task_1);
        assert_eq!(day_18_task_1, 36382392389406);

        let day_18_task_2 = aoc2020::day_18::task_2(&data_day_18);
        println!("AOC 2020, day 18, task 2: {}", day_18_task_2);
        assert_eq!(day_18_task_2, 381107029777968);
    } else {
        eprintln!("AOC 2020, day 18: Unable to read data for day 18. Skipping.");
    }

    // Day 19
    if let Ok(data_day_19) = std::fs::read_to_string("data/day19.txt") {
        let day_19_task_1 = aoc2020::day_19::task_1(&data_day_19);
        println!("\nAOC 2020, day 19, task 1: {}", day_19_task_1);
        assert_eq!(day_19_task_1, 176);

        // let day_19_task_2 = aoc2020::day_19::task_2(&data_day_19);
        // println!("AOC 2020, day 19, task 2: {}", day_19_task_2);
        // assert_eq!(day_19_task_2, );
    } else {
        eprintln!("AOC 2020, day 19: Unable to read data for day 19. Skipping.");
    }
}

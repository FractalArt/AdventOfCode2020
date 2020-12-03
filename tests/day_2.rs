use aoc2020::{self, read_data};

#[test]
fn test_day_2() {
    let data = read_data::<String, _>("data/day2.txt").unwrap();

    let task_1 = aoc2020::day_2::task_1(&data);
    assert_eq!(task_1, 528);

    let task_2 = aoc2020::day_2::task_2(&data);
    assert_eq!(task_2, 497);
}

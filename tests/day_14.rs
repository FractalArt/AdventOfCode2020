use aoc2020::{self, read_data};

#[test]
fn test_day_14() {
    let data = read_data::<String, _>("data/day14.txt").unwrap();

    let task_1 = aoc2020::day_14::task_1(&data);
    assert_eq!(task_1, 11884151942312);
    let task_2 = aoc2020::day_14::task_2(&data);
    assert_eq!(task_2, 2625449018811);
}

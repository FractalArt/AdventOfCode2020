use aoc2020::{self, read_data};

#[test]
fn test_day_8() {
    let data = read_data::<String, _>("data/day8.txt").unwrap();

    let task_1 = aoc2020::day_8::task_1(&data);
    assert_eq!(task_1, 2080);
}

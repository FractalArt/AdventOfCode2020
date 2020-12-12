use aoc2020::{self, read_data};

#[test]
fn test_day_12() {
    let data = read_data::<String, _>("data/day12.txt").unwrap();

    let task_1 = aoc2020::day_12::task_1(&data);
    assert_eq!(task_1, 1482);
}

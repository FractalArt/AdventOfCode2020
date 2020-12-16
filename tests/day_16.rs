use aoc2020::{self, read_data};

#[test]
fn test_day_16() {
    let data = read_data::<String, _>("data/day16.txt").unwrap();

    let task_1 = aoc2020::day_16::task_1(&data);
    assert_eq!(task_1, 23954);
    let task_2 = aoc2020::day_16::task_2(&data);
    assert_eq!(task_2, 453459307723);
}

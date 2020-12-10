use aoc2020::{self, read_data};

#[test]
fn test_day_9() {
    let data = read_data::<u64, _>("data/day9.txt").unwrap();

    let task_1 = aoc2020::day_9::task_1(&data, 25);
    assert_eq!(task_1, 257342611);

    let task_2 = aoc2020::day_9::task_2(&data, 25);
    assert_eq!(task_2, 35602097);
}

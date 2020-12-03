use aoc2020::{self, read_data};

#[test]
fn test_day_3() {
    let data = read_data::<String, _>("data/day3.txt").unwrap();

    let task_1 = aoc2020::day_3::task_1(&data, 3, 1);
    assert_eq!(task_1, 156);
    let task_2 = aoc2020::day_3::task_2(&data, &vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
    assert_eq!(task_2, 3521829480);
}

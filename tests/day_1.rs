use aoc2020::{self, read_data};

#[test]
fn test_day_1() {
    let data = read_data::<u32, _>("data/day1.txt").unwrap();
    let task_1 = aoc2020::day_1::day_1(&data, 2, 2020);
    assert_eq!(task_1, 270144);
    let task_2 = aoc2020::day_1::day_1(&data, 3, 2020);
    assert_eq!(task_2, 261342720);
}

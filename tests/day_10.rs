use aoc2020::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<u32, _>("data/day10.txt").unwrap();

    let task_1 = aoc2020::day_10::task_1(&data);
    assert_eq!(task_1, 2059);

    let task_2 = aoc2020::day_10::task_2(&data);
    assert_eq!(task_2, 86812553324672);
}

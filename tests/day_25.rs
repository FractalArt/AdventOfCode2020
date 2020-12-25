use aoc2020::{self, read_data};

#[test]
fn test_day_25() {
    let data = read_data::<usize, _>("data/day25.txt").unwrap();

    let task_1 = aoc2020::day_25::task_1(&data);
    assert_eq!(task_1, 18329280);
}

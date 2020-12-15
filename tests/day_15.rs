use aoc2020;

#[test]
fn test_day_15() {
    let data = [18, 11, 9, 0, 5, 1];

    let task_1 = aoc2020::day_15::task_1(&data, 2020);
    assert_eq!(task_1, 959);
    // let task_2 = aoc2020::day_15::task_2(&data);
    // assert_eq!(task_2, 2);
}

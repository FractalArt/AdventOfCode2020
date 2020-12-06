#[test]
fn test_day_6() {
    let data = std::fs::read_to_string("data/day6.txt").unwrap();

    let task_1 = aoc2020::day_6::task_1(&data);
    assert_eq!(task_1, 6542);
    let task_2 = aoc2020::day_6::task_2(&data);
    assert_eq!(task_2, 3299);
}

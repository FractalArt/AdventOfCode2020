use aoc2020;

#[test]
fn test_day_17() {
    let data = std::fs::read_to_string("data/day17.txt").unwrap();

    let task_1 = aoc2020::day_17::task_1(&data, 6);
    assert_eq!(task_1, 252);

    let task_2 = aoc2020::day_17::task_2(&data, 6);
    assert_eq!(task_2, 2160);
}

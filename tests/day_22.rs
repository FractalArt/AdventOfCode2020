use aoc2020;

#[test]
fn test_day_22() {
    let data = std::fs::read_to_string("data/day22.txt").unwrap();

    let task_1 = aoc2020::day_22::task_1(&data);
    assert_eq!(task_1, 31809);

    let task_2 = aoc2020::day_22::task_2(&data);
    assert_eq!(task_2, 32835);
}

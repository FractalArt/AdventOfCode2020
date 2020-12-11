#[test]
fn test_day_11() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();

    let task_1 = aoc2020::day_11::task_1(&data);
    assert_eq!(task_1, 2361);
}

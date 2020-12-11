#[test]
fn test_day_11() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();

    let task_1 = aoc2020::day_11::task_1_2(&data, &aoc2020::day_11::Strategy::Adjacent, 4);
    assert_eq!(task_1, 2361);

    let task_2 = aoc2020::day_11::task_1_2(&data, &aoc2020::day_11::Strategy::Visible, 5);
    assert_eq!(task_2, 2119);
}

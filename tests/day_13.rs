use aoc2020;

#[test]
fn test_day_13() {
    let data = std::fs::read_to_string("data/day13.txt").unwrap();

    let task_1 = aoc2020::day_13::task_1(&data);
    assert_eq!(task_1, 5257);

    let task_2 = aoc2020::day_13::task_2(&data);
    assert_eq!(task_2, 538703333547789);
}

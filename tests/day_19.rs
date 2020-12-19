use aoc2020;

#[test]
fn test_day_19() {
    let data = std::fs::read_to_string("data/day19.txt").unwrap();

    let task_1 = aoc2020::day_19::task_1(&data);
    assert_eq!(task_1, 176);

    // let task_2 = aoc2020::day_19::task_2(&data);
    // assert_eq!(task_2, );
}
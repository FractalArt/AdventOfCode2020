#[test]
fn test_day_4() {
    let content = std::fs::read_to_string("data/day4.txt").unwrap();

    let day_4_task_1 = aoc2020::day_4::task_1(&content);
    assert_eq!(day_4_task_1, 222);

    // let day_4_task_2 = aoc2020::day_4::task_2(&content);
    // assert_eq!(day_4_task_2, 222);
}

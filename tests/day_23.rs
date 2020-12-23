use aoc2020;

#[test]
fn test_day_23() {
    let data = [5, 2, 3, 7, 6, 4, 8, 1, 9];

    let task_1 = aoc2020::day_23::task_1(&data, 100);
    assert_eq!(task_1, 49576328);
    // let task_2 = aoc2020::day_23::task_1_2(&data);
    // assert_eq!(task_2, );
}

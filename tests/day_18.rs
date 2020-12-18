use aoc2020::{self, read_data};

#[test]
fn test_day_18() {
    let data = read_data::<String, _>("data/day18.txt").unwrap();

    let task_1 = aoc2020::day_18::task_1(&data);
    assert_eq!(task_1, 36382392389406);
    // let task_2 = aoc2020::day_18::task_2(&data);
    // assert_eq!(task_2, );
}

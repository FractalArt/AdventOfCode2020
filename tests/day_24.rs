use aoc2020::{self, read_data};

#[test]
fn test_day_24() {
    let data = read_data::<String, _>("data/day24.txt").unwrap();

    let task_1 = aoc2020::day_24::task_1(&data);
    assert_eq!(task_1, 479);
    // let task_2 = aoc2020::day_24::task_2(&data);
    // assert_eq!(task_2,);
}

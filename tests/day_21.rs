use aoc2020::{self, read_data};

#[test]
fn test_day_21() {
    let data = read_data::<String, _>("data/day21.txt").unwrap();

    let task_1 = aoc2020::day_21::task_1(&data);
    assert_eq!(task_1, 1930);
    let task_2 = aoc2020::day_21::task_2(&data);
    assert_eq!(task_2, "spcqmzfg,rpf,dzqlq,pflk,bltrbvz,xbdh,spql,bltzkxx");
}

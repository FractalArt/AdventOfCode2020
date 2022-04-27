# Advent of Code 2020

Attempting the [AOC 2020](https://adventofcode.com/2020/) problems as an excuse to practice Rust.

## Solutions

| Task | Status |
| ---- | :----: |
| Day 1, Part 1 | :heavy_check_mark: |
| Day 1, Part 2 | :heavy_check_mark: |
| Day 2, Part 1 | :heavy_check_mark: |
| Day 2, Part 2 | :heavy_check_mark: |
| Day 3, Part 1 | :heavy_check_mark: |
| Day 3, Part 2 | :heavy_check_mark: |
| Day 4, Part 1 | :heavy_check_mark: |
| Day 4, Part 2 | :heavy_check_mark: |
| Day 5, Part 1 | :heavy_check_mark: |
| Day 5, Part 2 | :heavy_check_mark: |
| Day 6, Part 1 | :heavy_check_mark: |
| Day 6, Part 2 | :heavy_check_mark: |
| Day 7, Part 1 | :heavy_check_mark: |
| Day 7, Part 2 | :heavy_check_mark: |
| Day 8, Part 1 | :heavy_check_mark: |
| Day 8, Part 2 | :heavy_check_mark: |
| Day 9, Part 1 | :heavy_check_mark: |
| Day 9, Part 2 | :heavy_check_mark: |
| Day 10, Part 1 | :heavy_check_mark: |
| Day 10, Part 2 | :heavy_check_mark: |
| Day 11, Part 1 | :heavy_check_mark: |
| Day 11, Part 2 | :heavy_check_mark: |
| Day 12, Part 1 | :heavy_check_mark: |
| Day 12, Part 2 | :heavy_check_mark: |
| Day 13, Part 1 | :heavy_check_mark: |
| Day 13, Part 2 | :heavy_check_mark: |
| Day 14, Part 1 | :heavy_check_mark: |
| Day 14, Part 2 | :heavy_check_mark: |
| Day 15, Part 1 | :heavy_check_mark: |
| Day 15, Part 2 | :heavy_check_mark: |
| Day 16, Part 1 | :heavy_check_mark: |
| Day 16, Part 2 | :heavy_check_mark: |
| Day 17, Part 1 | :heavy_check_mark: |
| Day 17, Part 2 | :heavy_check_mark: |
| Day 18, Part 1 | :heavy_check_mark: |
| Day 18, Part 2 | :heavy_check_mark: |
| Day 19, Part 1 | :heavy_check_mark: |
| Day 19, Part 2 | :x: |
| Day 20, Part 1 | :x: |
| Day 20, Part 2 | :x: |
| Day 21, Part 1 | :heavy_check_mark: |
| Day 21, Part 2 | :heavy_check_mark: |
| Day 22, Part 1 | :heavy_check_mark: |
| Day 22, Part 2 | :heavy_check_mark: |
| Day 23, Part 1 | :heavy_check_mark: |
| Day 23, Part 2 | :heavy_check_mark: |
| Day 24, Part 1 | :heavy_check_mark: |
| Day 24, Part 2 | :heavy_check_mark: |
| Day 25, Part 1 | :heavy_check_mark: |

***

## Comments

### Day 17

I did not know about [multi_cartesian_product](https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.multi_cartesian_product) which would have made the solution a bit cleaner and could probably be used to make it generic over the dimension. However, I will keep my initial solution in the repository the way it was before I started looking at other
solutions on `reddit` and simply keep this as a note in case it becomes useful in the future.

***
## Additional information

The main binary computes all the solutions and prints them to the screen. Its execution can be triggered
via the command

```bash
> cargo r --release
```

Isolated solutions to different tasks can be evaluated by running the corresponding integration tests

```bash
> cargo t --release test_day_<day>
```

Although sometimes code could be shortened by performing evaluations directly in the iterator chain,
I opted to delegate logic related to a specific task into a dedicated function. This allowed me to
use the AOC author's examples to generate unit tests that check the different parts of my solutions.

All tests can be run via the command

```bash
> cargo t --release
```

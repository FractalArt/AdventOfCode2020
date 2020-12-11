# Advent of Code 2020

![Build status](https://travis-ci.com/FractalArt/AdventOfCode2020.svg?branch=master)

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
| Day 10, Part 2 | :x: |
| Day 11, Part 1 | :heavy_check_mark: |

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
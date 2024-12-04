# Advent of Code 2024 - Python
For fun I also do some of the puzzles in python.


<!--- benchmarking table --->
## Benchmarks

Benchmarked total execution with hyperfine including file reading and argument parsing.

```
hyperfine --warmup 3 '.\.venv\Scripts\python.exe .\main.py -d 1 -p 1'
```

|            Day             |  Part 1   |  Part 2  | Total Part 1 | Total part 2 |
|:--------------------------:|:---------:|:--------:|:------------:|:------------:|
| [Day 1](./puzzles/day1.py) |  `641μs`  | `776μs`  |   `66.7ms`   |   `67.5ms`   |
| [Day 2](./puzzles/day2.py) | `2.15ms`  | `5.84ms` |   `83.1ms`   |   `87.6ms`   |
| [Day 3](./puzzles/day3.py) | `1.05ms`  | `1.47ms` |   `96.7ms`   |  `103.6ms`   |
| [Day 4](./puzzles/day4.py) | `11.83ms` | `3.67ms` |   `96.7ms`   |  `103.6ms`   |

**Total: 27.382ms**
<!--- benchmarking table --->

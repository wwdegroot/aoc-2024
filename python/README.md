# Advent of Code 2024 - Python
For fun I also do some of the puzzles in python.


<!--- benchmarking table --->
## Benchmarks

Benchmarked total execution with hyperfine including file reading and argument parsing.

```
hyperfine --warmup 3 '.\.venv\Scripts\python.exe .\main.py -d 1 -p 1'
```

|            Day             | Part 1  |  Part 2  | Total Part 1 | Total part 2 |
|:--------------------------:|:-------:|:--------:|:------------:|:------------:|
| [Day 1](./puzzles/day1.py) | `641μs` | `776μs`  |   `66.7ms`   |   `67.5ms`   |

**Total: 2.67ms**
<!--- benchmarking table --->
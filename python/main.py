import argparse
import time
from pathlib import Path

from puzzles import INPUTS, day9
from puzzles import day1, day2, day3, day4, day9


def read_file_contents(input_file: Path) -> str:
    return input_file.read_text(encoding="utf-8")


def print_elapsed_time(elapsed: int):
    elapsed_microseconds = round(elapsed / 1000)
    if 1000 < elapsed_microseconds < 1_000_000:
        elapsed_time = f"{elapsed_microseconds / 1000 :.2f} ms"
    elif elapsed_microseconds > 1_000_000:
        elapsed_time = f"{elapsed_microseconds / 1_000_000 :.2f} seconds"
    else:
        elapsed_time = f"{elapsed_microseconds} μs"
    print(f"Solved in {elapsed_time}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--day', type=int, default=1, help='Day')
    parser.add_argument('-p', '--part', type=int, default=1, help='Part')
    args = parser.parse_args()

    content = read_file_contents(INPUTS[args.day])
    start_time = time.perf_counter_ns()
    match args.day:
        case 1:
            day1.run(content, part=args.part)
        case 2:
            day2.run(content, part=args.part)
        case 3:
            day3.run(content, part=args.part)
        case 4:
            day4.run(content, part=args.part)
        case 9:
            day9.run(content, part=args.part)
        case _:
            raise ValueError(f"Invalid day input {args.day}. Enter number 1-25")

    end_time = time.perf_counter_ns()
    elapsed_time = end_time - start_time
    print_elapsed_time(elapsed_time)


if __name__ == '__main__':
    main()

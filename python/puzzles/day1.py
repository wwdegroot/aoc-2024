import itertools
import time
from collections import Counter

def parse_input(input: str) -> tuple[list[int], list[int]]:
    lines = input.splitlines()
    right = []
    left = []
    for line in lines:
        r,l = line.split()
        right.append(int(r))
        left.append(int(l))
    right.sort()
    left.sort()
    return right, left


def part_one(input: str) -> int:
    right, left = parse_input(input)
    result = 0
    for r, l in zip(right, left):
        result += abs(r-l)
    return result


def part_two(input: str) -> int:
    right, left = parse_input(input)
    result = 0
    counter = Counter(left)
    for r in right:
        result += r * counter[r]
    return result


def run(input: str, part: int) -> None:
    match part:
        case 1:
            result = part_one(input)
        case 2:
            result = part_two(input)
        case _:
            raise ValueError(f"Not a valid part number, 1,2 expected")

    print(f"Result part {part}: {result}")

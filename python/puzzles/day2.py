
def parse_input(input: str) -> list[list[int]]:
    return [[int(l) for l in line.split()] for line in input.splitlines()]


def safe_level(level: list[int]) -> bool:
    max_diff = False
    trend_negative = False
    trend_positive = False
    for i, l in enumerate(level, 1):
        if i < len(level):
            change = l - level[i]
            if abs(change) > 3:
                max_diff = True
            if change <= 0:
                trend_negative = True
            if change >= 0:
                trend_positive = True
    if trend_negative and trend_positive or max_diff:
        return False
    return True


def part_one(input: str) -> int:
    levels = parse_input(input)
    result = 0
    for level in levels:
        if safe_level(level):
            result += 1
    return result


def part_two(input: str) -> int:
    levels = parse_input(input)
    result = 0
    for level in levels:
        if safe_level(level):
            result += 1
            continue
        # dampening
        for i in range(len(level)):
            dlevel = level[:i] + level[i+1:]
            if safe_level(dlevel):
                result += 1
                break

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

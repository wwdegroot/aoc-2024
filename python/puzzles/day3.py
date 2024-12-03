import re


mul_pattern = re.compile(r"mul\(\d{1,3},\d{1,3}\)")
number_pattern = re.compile(r"(\d+)")
mul_pattern_do_dont = re.compile(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))")
enabled = True


def multiply_numbers(mul_string: str) -> int:
    a, b = number_pattern.findall(mul_string)
    return int(a)*int(b)


def part_one(input: str) -> int:
    return sum([multiply_numbers(p) for p in mul_pattern.findall(input)])


def handle_patterns(pattern: tuple[str, str, str]) -> int:
    global enabled
    nums, do, dont = pattern
    if nums != '' and enabled:
        return multiply_numbers(nums)
    if do == 'do()':
        enabled = True
    if dont == 'dont':
        enabled = False
    return 0


def part_two(input: str) -> int:
    return sum([handle_patterns(p) for p in mul_pattern_do_dont.findall(input)])


def run(input: str, part: int) -> None:
    match part:
        case 1:
            result = part_one(input)
        case 2:
            result = part_two(input)
        case _:
            raise ValueError(f"Not a valid part number, 1,2 expected")

    print(f"Result part {part}: {result}")

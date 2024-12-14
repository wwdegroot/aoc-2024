import math
from dataclasses import dataclass

@dataclass
class Coordinate:
    x: int
    y: int

@dataclass
class Instruction:
    start: tuple[int, int]
    velocity: tuple[int, int]

# constants
WIDTH = 101
HEIGTH = 103

TEST_WIDTH = 11
TEST_HEIGTH = 7

TEST_DATA = """p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"""


def parse_line(line: str) -> Instruction:
    start, velo = line.strip().split(" ")
    x, y = start.split("=")[1].split(',')
    vx, vy = velo.split("=")[1].split(',')
    return Instruction(start=(int(x), int(y)), velocity=(int(vx), int(vy)))


def parse_input(input: str) -> list[Instruction]:
    return [parse_line(line) for line in input.splitlines()]


def calcualte_position(start: tuple[int, int], vel: tuple[int, int],width: int, heigth: int, iter: int) -> tuple[int, int]:
    """
    We can calculate the final position by applying the vel x and y times iter and doing the modulo of the widht/heigth

    :param start:
    :param vel:
    :param iter:
    :return:
    """
    final_x = (start[0] + (vel[0] * iter)) % width
    final_y = (start[1] + (vel[1] * iter)) % heigth
    return int(final_x), int(final_y)


def calculate_final_positions(instructions: list[Instruction], width: int, heigth: int, iterations: int) -> list[tuple[int, int]]:
    final: list[tuple[int, int]] = []
    for ins in instructions:
        final.append(calcualte_position(ins.start, ins.velocity, width, heigth, iterations))
    return final


def score_quadrants(width: int, height: int, final: list[tuple[int, int]]) -> int:
    """
    Count the robots when either x or y is in a quadrant
    :param width:
    :param height:
    :param final:
    :return:
    """
    middle_x = math.floor(width / 2)
    middle_y = math.floor(height / 2)
    top_right, top_left, bottom_right, bottom_left = 0, 0, 0, 0
    for p in final:
        if p[0] < middle_x:
            if p[1] < middle_y:
                top_left += 1
            elif p[1] > middle_y:
                bottom_left += 1
        if p[0] > middle_x:
            if p[1] < middle_y:
                top_right += 1
            elif p[1] > middle_y:
                bottom_right += 1

    return top_right * top_left * bottom_right * bottom_left


def part_one(input: str) -> int:
    instructions = parse_input(input)
    final = calculate_final_positions(instructions, WIDTH, HEIGTH, iterations=100)
    result = score_quadrants(WIDTH, HEIGTH, final)
    return result


def part_two(input: str) -> int:
    result = 0
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


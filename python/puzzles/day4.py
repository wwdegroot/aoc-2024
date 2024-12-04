from collections import defaultdict


def parse_input(input: str) -> list[list[str]]:
    matrix = []
    for line in input.splitlines():
        matrix.append(list(line.strip()))
    return matrix


def check_bounds(x, y, size):
    return 0 <= x < size and 0 <= y < size


def find_xmas_matrix(matrix: list[list[str]]) -> int:
    count = 0
    width = len(matrix[0])
    height = len(matrix)
    for row in range(height):
        for col in range(width):
            if matrix[row][col] == 'X':
                # check for M and get back the coordinates
                M = find_neigbors(matrix, col, row, "M")
                if M:
                    for mr, mc in M:
                        # calculate the direction
                        rdir = row - mr
                        cdir = col - mc
                        # check if there is enough space left for XMAS
                        if check_bounds(mr - rdir * 2, mc - cdir * 2, width):
                            if matrix[mr - rdir][mc - cdir] == 'A' and matrix[mr - rdir * 2][mc - cdir * 2] == 'S':
                                count += 1
    return count


def find_neigbors(matrix: list[list[str]], col: int, row: int, char: str) -> None | list[tuple[int, int]]:
    directions = [(1, -1), (0, -1), (-1, -1), (-1, 0), (1, 0), (1, 1), (0, 1), (-1, 1)]
    dimension = len(matrix)
    results = defaultdict(list)
    for drow, dcol in directions:
        c = col - dcol
        r = row - drow
        if check_bounds(c, r, dimension):
            if char == matrix[r][c]:
                results[char].append((r, c))
    return results.get(char)


def search_mas(matrix: list[list[str]], col: int, row: int) -> bool:
    one = False
    two = False
    if check_bounds(col - 1, row - 1, len(matrix)) and check_bounds(col + 1, row + 1, len(matrix)):
        if (
            (matrix[row - 1][col - 1] == "M" and matrix[row + 1][col + 1] == "S")
            or
            (matrix[row - 1][col - 1] == "S" and matrix[row + 1][col + 1] == "M")
        ):
            one = True
        if (
            (matrix[row + 1][col - 1] == "M" and matrix[row - 1][col + 1] == "S")
            or
            (matrix[row + 1][col - 1] == "S" and matrix[row - 1][col + 1] == "M")
        ):
            two = True
    if one and two:
        return True


def find_mas_matrix(matrix: list[list[str]]) -> int:
    count = 0
    width = len(matrix[0])
    height = len(matrix)
    for row in range(height):
        for col in range(width):
            if matrix[row][col] == 'A':
                if search_mas(matrix, col, row):
                    count += 1
    return count


def part_one(input: str) -> int:
    matrix = parse_input(input)
    result = find_xmas_matrix(matrix)
    return result


def part_two(input: str) -> int:
    matrix = parse_input(input)
    result = find_mas_matrix(matrix)
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

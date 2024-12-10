from collections import deque

TEST_DATA = """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"""

def parse_input(puzzle_input: str):
    grid = []
    start_locations = []
    for row, line in enumerate(puzzle_input.splitlines()):
        numbers = [int(num) for num in line]
        grid.append(numbers)
        col = 0
        for n in line:
            if n == '0':
                start_locations.append((row, col))
            col += 1
    return grid, start_locations


def find_paths_to_peaks(grid, start_row, start_col) -> int:
    rows, cols = len(grid), len(grid[0])
    queue = deque([(start_row, start_col)])
    visited = set()
    paths = set()
    trailheads = 0
    def is_in_bounds(c, r):
        return 0 <= r < rows and 0 <= c < cols

    while queue:
        row, col = queue.popleft()

        current_value = grid[row][col]
        visited.add((row, col))
        if current_value == 9:
            trailheads += 1
            paths.add((row, col))

        # Explore neighbors and check for delta 1 and add to queue
        for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nr, nc = row + dr, col + dc
            if is_in_bounds(nc, nr) and (nr, nc) not in visited:
                nval = grid[nr][nc]
                if (nval - current_value) == 1:
                    queue.append((nr, nc))
    return len(paths), trailheads


def part_one(input: str) -> int:
    grid, start_locations = parse_input(input)
    result = 0
    for loc in start_locations:
        p, _ = find_paths_to_peaks(grid, loc[0], loc[1])
        result += p
    return result


def part_two(input: str) -> int:
    grid, start_locations = parse_input(input)

    result = 0
    for loc in start_locations:
        _, p = find_paths_to_peaks(grid, loc[0], loc[1])
        result += p
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

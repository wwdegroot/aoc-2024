from collections import deque

TEST_DATA = """RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE2"""


TEST_DATA2 = """EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"""



def parse_input_grids(puzzle_input: str) -> list[list[str]]:
    plots = []
    for line in puzzle_input.splitlines():
        plots.append([l.strip() for l in line])
    return plots


def find_regions(plots: list[list[str]]) -> list[list[tuple[int, int]]]:
    """
    Breadth First Search Implementation
    """

    # Rows and columns of the map
    rows = len(plots)
    cols = len(plots[0])

    # keep track of visited cells
    visited = {(r,c): False for r in range(rows) for c in range(cols)}

    # all unique regions
    regions: list[list[tuple[int, int]]] = []

    for pos in visited.keys():
        # skip visited cells
        if visited[pos]:
            continue
        q = deque()
        region = []
        # update state
        last_seen = plots[pos[0]][pos[1]]
        visited[pos] = True
        region.append(pos)
        q.append(pos)
        while q:
            # Dequeue the front node
            r, c = q.popleft()

            # Check if the adjacent cells have the same character
            # check below
            if r + 1 < rows and plots[r + 1][c] == last_seen and not visited[(r + 1, c)]:
                visited[(r + 1, c)] = True
                region.append((r + 1, c))
                q.append((r + 1, c))
            # check above
            if r - 1 >= 0 and plots[r - 1][c] == last_seen and not visited[(r -1, c)]:
                visited[(r - 1, c)] = True
                region.append((r - 1, c))
                q.append((r - 1, c))
            # check right
            if c + 1 < cols and plots[r][c + 1] == last_seen and not visited[(r, c + 1)]:
                visited[(r, c + 1)] = True
                region.append((r, c + 1))
                q.append((r, c + 1))
            # check left
            if c - 1 >= 0 and plots[r][c - 1] == last_seen and not visited[(r, c - 1)]:
                visited[(r, c - 1)] = True
                region.append((r, c - 1))
                q.append((r, c - 1))

        regions.append(region)

    return regions


def calculate_fencing(regions: list[list[tuple[int, int]]]) -> int:
    result = 0
    for region in regions:
        area = len(region)
        fencing = 0
        # check how many neigbors and substract from amount of sides
        for cell in region:
            fences = 4
            for dr, dc in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                if (cell[0] + dr, cell[1] + dc) in region:
                    fences -= 1
            fencing += fences

        result += area * fencing
    return result



def part_one(input: str) -> int:
    plots = parse_input_grids(input)
    regions = find_regions(plots)
    result = calculate_fencing(regions)
    return result


def part_two(input: str) -> int:
    plots = parse_input_grids(TEST_DATA)
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




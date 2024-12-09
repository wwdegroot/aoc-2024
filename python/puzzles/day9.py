from itertools import batched
from collections import deque
from dataclasses import dataclass
from typing import Deque


@dataclass
class FileBlock:
    index: int
    fileblock: int
    space: int = 0


type FileBlocks = list[FileBlock]


TEST_DATA = "2333133121414131402"


def parse_input(input: str) -> FileBlocks:
    disk_map = []
    chunked_input = list(batched(input, n=2))
    for i, dm in enumerate(chunked_input):
        if len(dm) == 1:
            dm = (dm[0], 0)
        # print(f"file_id={i}")
        # print(f"file_block={dm[0]}, free_space={dm[1]}")
        disk_map.append(FileBlock(index=i, fileblock=int(dm[0]), space=int(dm[1])))
    return disk_map


def disk_checksum_blocks(blocks: FileBlocks) -> int:
    checksum = 0
    checksum_list = []
    blocks: Deque[FileBlock] = deque(blocks)

    while blocks:
        left = blocks.popleft()
        for i in range(left.fileblock):
            checksum_list.append(left.index)
        while blocks and left.space > 0:
            right = blocks.pop()
            if left.space < right.fileblock or left.space == right.fileblock:
                available = left.space
            if left.space > right.fileblock:
                available = right.fileblock
            for _ in range(available):
                checksum_list.append(right.index)
            right.fileblock -= available
            left.space -= available
            if right.fileblock > 0:
                blocks.append(right)

    for ix, fb in enumerate(checksum_list):
        checksum += ix * fb
    return checksum


def part_one(input: str) -> int:
    disk_space = parse_input(input)
    result = disk_checksum_blocks(disk_space)
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

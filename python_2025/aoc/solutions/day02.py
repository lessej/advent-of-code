from aoc_utils import load_input
from functools import reduce
from operator import add


def get_ids(lines):
    ranges = [range for range in lines.split(",")]
    return [[int(id) for id in range.strip().split("-")] for range in ranges]


def part_1():
    ids = get_ids(load_input(2))
    invalid_ids = []
    for [start, end] in ids:
        for curr in range(start, end + 1):
            id = str(curr)
            half = len(id) // 2
            l = id[:half]
            r = id[half:]
            if l == r:
                invalid_ids.append(curr)

    return reduce(add, invalid_ids)


def made_of_substrings(id):
    half = len(id) // 2
    for sub_len in range(1, half + 1):
        sub = id[:sub_len]
        r = sub_len
        while r < len(id):
            if sub != id[r : r + sub_len]:
                break
            r += sub_len
        if r == len(id):
            return True
    return False


def part_2():
    ids = get_ids(load_input(2))
    invalid_ids = []

    for [start, end] in ids:
        for curr in range(start, end + 1):
            if made_of_substrings(str(curr)):
                invalid_ids.append(curr)

    return reduce(add, invalid_ids)

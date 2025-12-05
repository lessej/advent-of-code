from aoc_utils import load_input
from functools import reduce
from operator import add


def merge_ranges(fresh):
    ranges = fresh
    i = 0
    while i < len(ranges):
        [si, ei] = ranges[i]
        j = i + 1
        while j < len(ranges):
            [sj, ej] = ranges[j]
            if can_merge([si, ei], [sj, ej]):
                merged = merge([si, ei], [sj, ej])
                ranges.pop(i)
                ranges.pop(j - 1)
                ranges.insert(0, merged)
                i = -1
                break
            j += 1
        i += 1
    return ranges


def can_merge(r1, r2):
    return (
        (r1[0] <= r2[0] and r1[1] >= r2[0])
        or (r1[0] <= r2[1] and r1[1] >= r2[1])
        or (r1[0] >= r2[0] and r1[1] <= r2[1])
    )


def merge(r1, r2):
    l = min(r1[0], r2[0])
    r = max(r1[1], r2[1])
    return [l, r]


def into_parts(lines):
    parts = [part.split("\n") for part in lines.split("\n\n")]
    return [[item for item in part if item != ""] for part in parts]


def part_1():
    lines = load_input(5)
    parts = into_parts(lines)

    fresh = [[int(anchor) for anchor in part.split("-")] for part in parts[0]]
    avail = set([int(ingredient) for ingredient in parts[1]])

    ranges = merge_ranges(fresh)
    avail_fresh = set()

    for a in avail:
        for sr, er in ranges:
            if a >= sr and a <= er:
                avail_fresh.add(a)

    return len(avail_fresh)


def part_2():
    lines = load_input(5)
    parts = into_parts(lines)

    fresh = [[int(anchor) for anchor in part.split("-")] for part in parts[0]]

    ranges = merge_ranges(fresh)
    return reduce(add, [r[1] - r[0] + 1 for r in ranges])

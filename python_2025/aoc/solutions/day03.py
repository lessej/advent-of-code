from aoc_utils import load_input
from functools import reduce
from operator import add


def find_max_in_range(line, start, end):
    m = start
    for i in range(start, end):
        if int(line[i]) > int(line[m]):
            m = i
    return m


def find_max_in_line(line, digits):
    s = 0
    e = len(line) - digits
    max = ""
    while e < len(line):
        lm = find_max_in_range(line, s, e)
        max += line[lm]
        e += 1
        s = lm + 1
    return int(max)


def part_1():
    lines = load_input(3, as_lines=True)
    maxes = []
    for line in lines:
        m = find_max_in_line(line, 2)
        maxes.append(m)
    return reduce(add, maxes)


def part_2():
    lines = load_input(3, as_lines=True)
    maxes = []
    for line in lines:
        m = find_max_in_line(line, 12)
        maxes.append(m)

    return reduce(add, maxes)

from aoc_utils import load_input


def get_parts(line):
    return line[0], int(line[1:])


def part_1():
    lines = load_input(1, as_lines=True)
    pos = 50
    cycles = 0

    for line in lines:
        dir, clicks = get_parts(line)
        clicks = clicks if dir == "R" else -clicks

        pos = (pos + clicks) % 100
        if pos == 0:
            cycles += 1

    return cycles


def part_2():
    lines = load_input(1, as_lines=True)
    pos = 50
    cycles = 0

    for line in lines:
        dir, clicks = get_parts(line)
        if dir == "R":
            cycles += (clicks + pos) // 100
        if dir == "L":
            cycles += ((100 - pos) % 100 + clicks) // 100

        if dir == "L":
            clicks *= -1
        pos = (pos + clicks) % 100

    return cycles

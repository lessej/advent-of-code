from aoc_utils import load_input


def into_coords(lines):
    lines = [line.split(",") for line in lines]
    return [(int(line[0]), int(line[1])) for line in lines]


def area(v1, v2):
    a, b = v1
    x, y = v2
    return (abs(a - x) + 1) * (abs(b - y) + 1)


def get_maxes(coords):
    maxes = []
    for i in range(len(coords)):
        for j in range(i + 1, len(coords)):
            a = area(coords[i], coords[j])
            maxes.append((a, (coords[i], coords[j])))
    maxes.sort(key=lambda a: -a[0])
    return maxes


def part_1():
    lines = load_input(9, as_lines=True)
    coords = into_coords(lines)
    maxes = get_maxes(coords)

    return maxes[0][0]


def encloses(target, corner_1, corner_2):
    (a, b) = corner_1
    (x, y) = corner_2
    (row, col) = target

    return row > min(a, x) and row < max(a, x) and col > min(b, y) and col < max(b, y)


def part_2():
    lines = load_input(9, as_lines=True)
    coords = into_coords(lines)
    maxes = get_maxes(coords)
    coords.append(coords[0])

    for max in maxes:
        is_possible = True
        for coord in coords:
            if coord == max[1][0] or coord == max[1][1]:
                continue
            if encloses(coord, max[1][0], max[1][1]):
                is_possible = False
                break
        if is_possible:
            for ci in range(1, len(coords)):
                mid_x = (coords[ci - 1][0] + coords[ci][0]) // 2
                mid_y = (coords[ci - 1][1] + coords[ci][1]) // 2
                if encloses((mid_x, mid_y), max[1][0], max[1][1]):
                    is_possible = False
                    break
        if is_possible:
            return max[0]
    raise Exception("Couldn't find a solution")

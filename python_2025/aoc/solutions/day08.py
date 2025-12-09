from aoc_utils import load_input
from math import sqrt, inf
from functools import reduce
from operator import mul


def into_coords(lines):
    lines = [line.split(",") for line in lines]
    return [(int(line[0]), int(line[1]), int(line[2])) for line in lines]


def eu_dist(v1, v2):
    a, b, c = v1
    x, y, z = v2
    return sqrt((a - x) ** 2 + (b - y) ** 2 + (c - z) ** 2)


def part_1():
    lines = load_input(8, as_lines=True)
    coords = into_coords(lines)

    mat = [[0.0] * len(coords) for _ in coords]
    for i in range(len(mat)):
        for j in range(len(mat[0])):
            if i == j:
                mat[i][j] = inf
            else:
                mat[i][j] = eu_dist(coords[i], coords[j])

    circuits = [-1 for _ in range(len(coords))]
    curr = 0
    for _ in range(1000):
        min = (inf, (0, 0))
        for i in range(len(mat)):
            for j in range(len(mat[0])):
                if mat[i][j] < min[0]:
                    min = (mat[i][j], (i, j))
        x, y = min[1]
        mat[x][y] = inf
        mat[y][x] = inf
        if circuits[x] == -1 and circuits[y] == -1:
            circuits[x] = curr
            circuits[y] = curr
            curr += 1
        elif circuits[x] == -1:
            circuits[x] = circuits[y]
        elif circuits[y] == -1:
            circuits[y] = circuits[x]
        else:
            f_circ = circuits[x]
            t_circ = circuits[y]

            for i in range(len(circuits)):
                if circuits[i] == f_circ:
                    circuits[i] = t_circ
    counts = {}
    for circ in circuits:
        if circ < 0:
            continue
        if counts.get(circ, None) == None:
            counts[circ] = 0
        counts[circ] += 1
    counts = [counts[k] for k in counts]
    counts.sort()

    return reduce(mul, counts[-3:])


def part_2():
    lines = load_input(8, as_lines=True)
    coords = into_coords(lines)

    mat = [[0.0] * len(coords) for _ in coords]
    for i in range(len(mat)):
        for j in range(len(mat[0])):
            if i == j:
                mat[i][j] = inf
            else:
                mat[i][j] = eu_dist(coords[i], coords[j])

    circuits = [-i for i in range(len(coords))]
    curr = 0
    last = (0, 0)
    while len(set(circuits)) != 1:
        smallest = (inf, (0, 0))
        for i in range(len(mat)):
            for j in range(len(mat[0])):
                if mat[i][j] < smallest[0]:
                    smallest = (mat[i][j], (i, j))
        x, y = smallest[1]
        last = (x, y)
        mat[x][y] = inf
        mat[y][x] = inf
        if circuits[x] < 0 and circuits[y] < 0:
            circuits[x] = curr
            circuits[y] = curr
            curr += 1
        elif circuits[x] < 0:
            circuits[x] = circuits[y]
        elif circuits[y] < 0:
            circuits[y] = circuits[x]
        else:
            f_circ = circuits[x]
            t_circ = circuits[y]

            for i in range(len(circuits)):
                if circuits[i] == f_circ:
                    circuits[i] = t_circ
    return coords[last[0]][0] * coords[last[1]][0]

from aoc_utils import load_input
import sys

sys.setrecursionlimit(20000)


offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]


def in_grid(r, c, grid):
    return r >= 0 and r < len(grid) and c >= 0 and c < len(grid[0])


# For fun, an OCaml-y recursive solution with higher order functions.
# It could be done in fewer lines the way part 2 is done.
def part_1():
    grid = load_input(4, as_lines=True)

    def rec_loop(row, col, accessible):
        if row >= len(grid):
            return accessible
        if col >= len(grid[row]):
            return rec_loop(row + 1, 0, accessible)
        if grid[row][col] == "@":

            def rec_neighbors(neighbors, paper_count):
                if len(neighbors) <= 0:
                    return paper_count
                dx = row + neighbors[0][0]
                dy = col + neighbors[0][1]
                if in_grid(dx, dy, grid) and grid[dx][dy] == "@":
                    paper_count += 1
                return rec_neighbors(neighbors[1:], paper_count)

            neighbor_count = rec_neighbors(offsets, 0)
            accessible += 1 if neighbor_count < 4 else 0
        return rec_loop(row, col + 1, accessible)

    return rec_loop(0, 0, 0)


def is_accessible(row, col, grid):
    paper = 0
    for neighbor in offsets:
        if paper >= 4:
            return False
        nx = row + neighbor[0]
        ny = col + neighbor[1]
        if in_grid(nx, ny, grid) and grid[nx][ny] == "@":
            paper += 1
    return paper < 4


def part_2():
    grid = load_input(4, as_lines=True)
    grid = [list(row) for row in grid]

    removed = 0
    can_continue = True
    while can_continue:
        can_continue = False
        for row in range(len(grid)):
            for col in range(len(grid[row])):
                if grid[row][col] == "@" and is_accessible(row, col, grid):
                    grid[row][col] = "x"
                    removed += 1
                    can_continue = True

    return removed

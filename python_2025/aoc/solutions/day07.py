from aoc_utils import load_input


def oob(x, y, grid):
    return x < 0 or x >= len(grid) or y < 0 or y >= len(grid[0])


def part_1():
    grid = load_input(7, as_lines=True)
    start = (0, len(grid[0]) // 2 - 1)

    visited = {}
    q = []
    q.append(start)
    splits = 0
    while len(q) > 0:
        (row, col) = q.pop()
        key = f"{row}-{col}"
        if oob(row, col, grid) or visited.get(key, False):
            continue
        visited[key] = True
        if grid[row][col] == "^":
            q.append((row, col - 1))
            q.append((row, col + 1))
            splits += 1
        else:
            q.append((row + 1, col))
    return splits


def part_2():
    grid = [list(row)[:-1] for row in load_input(7, as_lines=True)]

    done = [[1 if c == "S" else 0 for c in grid[0]]]
    splits = 0
    for row in grid[1:]:
        curr = [-1 if c == "^" else 0 for c in row]
        for i, c in enumerate(row):
            above = done[-1][i]
            if above > 0:
                if c == "^":
                    splits += 1
                    curr[i - 1] += above
                    curr[i + 1] += above
                else:
                    curr[i] += above
        done.append(curr)
    return sum([i for i in done[-1] if i > 0])

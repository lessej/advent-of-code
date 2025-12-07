from aoc_utils import load_input
from operator import add, mul


def into_columns(lines):
    lines = [line.split() for line in lines]

    columns = []
    for i in range(len(lines[0])):
        column = []
        for j in range(len(lines)):
            column.insert(0, (lines[j][i]))
        columns.append(column)

    return columns


# Thought this might be a DP problem, but it's faster if I don't cache
# the results as I go lol
def part_1():
    lines = load_input(6, as_lines=True)
    columns = into_columns(lines)

    total = 0
    for column in columns:
        op = add if column[0] == "+" else mul
        carry = 0 if column[0] == "+" else 1
        for num in column[1:]:
            carry = op(carry, int(num))
        total += carry

    return total


def chunk_starts(lines):
    chunk_starts = []
    for i in range(len(lines[0])):
        is_empty = True
        for j in range(len(lines)):
            if lines[j][i] != " ":
                is_empty = False
                break
        if is_empty:
            chunk_starts.append(i)
    return chunk_starts


# Probably the operation and the chunk splitting could be
# done in a single loop but I'm too lazy to do that
def part_2():
    lines = load_input(6, as_lines=True)
    chunks = chunk_starts(lines)
    chunks.append(len(lines[0]) - 1)

    i = 0
    total = 0
    while i < len(chunks):
        start = chunks[i]
        end = chunks[i - 1] if i > 0 else -1

        op_str = lines[len(lines) - 1][end + 1]
        op = add if op_str == "+" else mul
        carry = 0 if op_str == "+" else 1

        for c in range(start - 1, end, -1):
            col_val = ""
            for r in range(len(lines) - 1):
                if lines[r][c] != " ":
                    col_val += lines[r][c]
            carry = op(carry, int(col_val.strip()))
        total += carry
        i += 1
    return total

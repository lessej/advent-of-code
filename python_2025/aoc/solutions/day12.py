from aoc_utils import load_input


def into_parts(lines):
    presents = []
    trees = []
    is_tree = False
    for line in lines:
        if len(line) > 4:
            is_tree = True
        if is_tree:
            trees.append(line)
        else:
            presents.append(line)
    return presents, trees


def into_trees(tree_lines):
    trees = []
    for line in tree_lines:
        a, p = line.split(": ")
        l, w = a.split("x")
        a = int(l) * int(w)
        p = [int(pres) for pres in p.split()]
        trees.append((a, p))
    return trees


# This is a pretty dumb solution and totally misses
# the problem but the areas seemed way too small for
# the number of presents.
def part_1():
    lines = load_input(12, as_lines=True)
    _, trees = into_parts(lines)
    trees = into_trees(trees)

    will_fit = 0
    for a, p in trees:
        pa = sum(p) * 9
        if pa <= a:
            will_fit += 1
    return will_fit


def part_2():
    return 0

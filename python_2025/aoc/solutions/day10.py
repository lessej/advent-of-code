from aoc_utils import load_input
from scipy.optimize import milp, LinearConstraint, Bounds
import numpy as np


def into_parts(lines):
    lines = [line.split() for line in lines]
    machines = []
    for line in lines:
        indicators = [False if i == "." else True for i in line[0][1:-1]]
        buttons = line[1:-1]
        buttons_list = []
        button_maps = []
        for b in buttons:
            button_map = [False for _ in range(len(indicators))]
            on_idxs = [int(i) for i in b[1:-1].split(",")]
            buttons_list.append(on_idxs)
            for idx in on_idxs:
                button_map[idx] = True
            button_maps.append(button_map)
        joltages = [int(j) for j in line[-1][1:-1].split(",")]
        machines.append((indicators, button_maps, joltages, buttons_list))

    return machines


def part_1():
    lines = load_input(10, as_lines=True)
    machines = into_parts(lines)

    def bfs_press(machine):
        indicators, buttons, _ = machine

        q = []
        seen = set()
        q.append(([False for _ in indicators], 0))

        while len(q) > 0:
            curr, presses = q.pop(0)
            key = ",".join(str(i) for i in curr)
            if key in seen:
                continue
            seen.add(key)
            for b in buttons:
                next = [a ^ b for a, b in zip(curr, b)]
                if next == indicators:
                    return presses + 1
                q.append((next, presses + 1))
        return -1

    return sum(bfs_press(m) for m in machines)


# Credit to: https://github.com/alexprengere/advent_of_code/blob/master/2025/10/python/main.py
# I had never heard of milp
def part_2():
    lines = load_input(10, as_lines=True)
    machines = into_parts(lines)

    res = 0
    for m in machines:
        _, _, joltages, buttons_list = m
        B = np.zeros((len(joltages), len(buttons_list)), dtype=int)
        for i, b in enumerate(buttons_list):
            for j in b:
                B[j, i] = 1

        J = np.array(joltages, dtype=float)
        n_vars = len(buttons_list)
        c = np.ones(len(buttons_list), dtype=float)
        constraints = LinearConstraint(B, lb=J, ub=J)
        bounds = Bounds(lb=np.zeros(n_vars), ub=np.full(n_vars, np.inf))
        integrality = np.ones(n_vars, dtype=int)

        min_presses = milp(
            c=c, integrality=integrality, constraints=constraints, bounds=bounds
        )
        if not min_presses.success:
            raise Exception("Couldn't find a solution")

        res += sum(np.rint(min_presses.x).astype(int))

    return res

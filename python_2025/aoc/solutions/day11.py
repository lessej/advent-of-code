from aoc_utils import load_input


def into_graph(lines):
    nodes = [(n, nn.split()) for line in lines for n, nn in [line.split(": ")]]
    graph = {}
    for n, nn in nodes:
        if n not in graph:
            graph[n] = set()
        graph[n] = set(nn).union(graph[n])
    return graph


def dfs(graph, start, end, seen):
    cache = {}

    def rec(start, end, seen):
        fft, dac = seen
        key = f"{start}-{fft}-{dac}"
        if key in cache:
            return cache[key]
        if start == end:
            return 1 if fft and dac else 0
        from_node = sum(
            rec(n, end, (fft or n == "fft", dac or n == "dac")) for n in graph[start]
        )
        cache[key] = from_node
        return from_node

    return rec(start, end, seen)


def part_1():
    lines = load_input(11, as_lines=True)
    graph = into_graph(lines)
    return dfs(graph, "you", "out", (True, True))


def part_2():
    lines = load_input(11, as_lines=True)
    graph = into_graph(lines)
    return dfs(graph, "svr", "out", (False, False))

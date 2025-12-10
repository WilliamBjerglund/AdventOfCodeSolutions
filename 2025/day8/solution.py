import math
from collections import Counter


def solve_junction_circuits(junction_boxes):
    num_boxes = len(junction_boxes)

    parent = list(range(num_boxes))

    def find_root(box_index):
        if parent[box_index] != box_index:
            parent[box_index] = find_root(parent[box_index])
        return parent[box_index]

    def union_circuits(box1, box2):
        parent[find_root(box1)] = find_root(box2)

    edges = []
    for i in range(num_boxes):
        for j in range(i + 1, num_boxes):
            distance = math.sqrt(
                sum(
                    (junction_boxes[i][k] - junction_boxes[j][k]) ** 2 for k in range(3)
                )
            )
            edges.append((distance, i, j))

    edges.sort()
    for _, i, j in edges[:1000]:
        union_circuits(i, j)

    circuit_sizes = Counter(find_root(i) for i in range(num_boxes)).values()
    return math.prod(sorted(circuit_sizes)[-3:])


def solve_complete_circuit(junction_boxes):
    num_boxes = len(junction_boxes)

    parent = list(range(num_boxes))

    def find_root(box_index):
        if parent[box_index] != box_index:
            parent[box_index] = find_root(parent[box_index])
        return parent[box_index]

    def union_circuits(box1, box2):
        parent[find_root(box1)] = find_root(box2)

    edges = []
    for i in range(num_boxes):
        for j in range(i + 1, num_boxes):
            distance = math.sqrt(
                sum(
                    (junction_boxes[i][k] - junction_boxes[j][k]) ** 2 for k in range(3)
                )
            )
            edges.append((distance, i, j))

    edges.sort()
    for _, i, j in edges:
        if find_root(i) != find_root(j):
            union_circuits(i, j)
            if len(set(find_root(k) for k in range(num_boxes))) == 1:
                return junction_boxes[i][0] * junction_boxes[j][0]

    return 0


if __name__ == "__main__":
    with open("2025/day8/input.txt") as f:
        junction_boxes = [tuple(map(int, line.split(","))) for line in f]

    result = solve_junction_circuits(junction_boxes)
    result2 = solve_complete_circuit(junction_boxes)
    print(
        f"Product of three largest circuits: {result}, product of complete circuit: {result2}"
    )

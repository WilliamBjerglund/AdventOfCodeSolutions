"""
Day 4: Printing Department
"""


def available_rolls_count(grid: list[str]) -> int:
    total_rows = len(grid)
    total_cols = len(grid[0])

    adjacent_directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    accessible = 0  # count of accessible rolls

    for r in range(total_rows):  # iterate through each row
        for c in range(total_cols):  # iterate through each column
            if grid[r][c] != "@":
                continue
            adjacent_rolls = 0
            for dr, dc in adjacent_directions:  # check all eight directions
                nr, nc = r + dr, c + dc  # new row and column
                if 0 <= nr < total_rows and 0 <= nc < total_cols:
                    if grid[nr][nc] == "@":
                        adjacent_rolls += 1
            if adjacent_rolls < 4:  # if fewer than 4 adjacent rolls
                accessible += 1
    return accessible


def remove_and_check_new_map_if_more_become_available(grid: list[str]) -> int:
    """
    This time we want to essentially the same this time once a toilet roll can be accessed, it is removed from the map.
    """
    # Convert to list of lists so we can modify it
    grid = [list(row) for row in grid]
    total_rows = len(grid)
    total_cols = len(grid[0])
    adjacent_directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]

    total_removed = 0

    while True:
        # Find all accessible rolls in current state
        accessible_positions = []
        for r in range(total_rows):
            for c in range(total_cols):
                if grid[r][c] != "@":
                    continue
                adjacent_rolls = 0
                for dr, dc in adjacent_directions:
                    nr, nc = r + dr, c + dc
                    if 0 <= nr < total_rows and 0 <= nc < total_cols:
                        if grid[nr][nc] == "@":
                            adjacent_rolls += 1
                if adjacent_rolls < 4:
                    accessible_positions.append((r, c))

        # If no accessible rolls found, we're done
        if not accessible_positions:
            break

        # Remove all accessible rolls
        for r, c in accessible_positions:
            grid[r][c] = "."

        total_removed += len(accessible_positions)

    return total_removed


if __name__ == "__main__":
    with open("2025/day4/input.txt", "r") as f:
        warehouse_grid = [line.strip() for line in f.readlines()]
    result = available_rolls_count(warehouse_grid)
    result2 = remove_and_check_new_map_if_more_become_available(warehouse_grid)
    print(
        "\n",
        f"Number of accessible rolls: {result}, Number of rolls removed after cascading effect: {result2}",
    )

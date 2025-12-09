"""
day 7: Laboratories
"""


def How_many_splits(grid):
    grid_height = len(grid)
    grid_width = len(grid[0])

    # Find the starting position
    for row_index, row_string in enumerate(grid):
        start_col_index = row_string.find("S")
        if start_col_index != -1:
            start_row_index, start_col_index = row_index, start_col_index
            break

    # Active beams are represented by column indices in the current row
    active_beam_columns = set()
    if start_row_index + 1 < grid_height:
        # Beams start one row below S, same column
        active_beam_columns.add(start_col_index)

    total_split_events = 0

    # Move row by row, downward
    for row_index in range(start_row_index + 1, grid_height):
        next_row_active_beam_columns = set()

        for beam_col in active_beam_columns:
            cell_symbol = grid[row_index][beam_col]

            if cell_symbol == "^":
                # Beam stops and splits left/right
                total_split_events += 1
                if beam_col - 1 >= 0:
                    next_row_active_beam_columns.add(beam_col - 1)
                if beam_col + 1 < grid_width:
                    next_row_active_beam_columns.add(beam_col + 1)
            else:
                # Beam continues straight down
                next_row_active_beam_columns.add(beam_col)

        active_beam_columns = next_row_active_beam_columns
        if not active_beam_columns:
            break

    return total_split_events


"""
Part 2: Many timelines:
"""


def How_many_timelines(grid):
    grid_height = len(grid)
    grid_width = len(grid[0])

    # find starting position
    for row_index, row_string in enumerate(grid):
        start_col_index = row_string.find("S")
        if start_col_index != -1:
            start_row_index, start_col_index = row_index, start_col_index
            break

    # Initialize DP table
    timeline_dp = [[0] * grid_width for _ in range(grid_height)]

    if start_row_index + 1 < grid_height:
        timeline_dp[start_row_index + 1][
            start_col_index
        ] = 1  # Start with one timeline below S

    for row_index in range(start_row_index + 1, grid_height):
        for col_index in range(grid_width):
            if timeline_dp[row_index][col_index] == 0:
                continue

            cell_symbol = grid[row_index][col_index]

            if cell_symbol == ".":
                if row_index + 1 < grid_height:
                    timeline_dp[row_index + 1][col_index] += timeline_dp[row_index][
                        col_index
                    ]

            elif cell_symbol == "^":
                # split left timeline
                if col_index - 1 >= 0:
                    timeline_dp[row_index + 1][col_index - 1] += timeline_dp[row_index][
                        col_index
                    ]
                # split right timeline
                if col_index + 1 < grid_width:
                    timeline_dp[row_index + 1][col_index + 1] += timeline_dp[row_index][
                        col_index
                    ]

    return sum(timeline_dp[grid_height - 1][c] for c in range(grid_width))


if __name__ == "__main__":
    with open("2025/day7/input.txt") as f:
        grid = [line.strip() for line in f.readlines()]

    result = How_many_splits(grid)
    result2 = How_many_timelines(grid)
    print(f"Number of splits: {result}, Number of timelines: {result2}")

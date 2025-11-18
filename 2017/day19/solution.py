def path_follower(routing_diagram):
    grid = [list(line) for line in routing_diagram.splitlines()]
    current_row, current_col = find_starting_position(grid)
    direction_row, direction_col = (1, 0)  # move downwards
    collected_letters = []
    step_counter = 0

    while within_bounds(current_row, current_col, grid) and not empty_cell(current_row, current_col, grid):
        step_counter += 1
        current_cell = grid[current_row][current_col]
        if current_cell.isalpha():
            collected_letters.append(current_cell)
        elif current_cell == '+':
            direction_row, direction_col = new_direction(current_row, current_col, (direction_row, direction_col), grid)
        
        current_row += direction_row
        current_col += direction_col
    return ''.join(collected_letters), step_counter
                

def find_starting_position(grid):
    """ This function start finder """
    return 0, grid[0].index('|')

def within_bounds(row, col, grid):
    return 0 <= row < len(grid) and 0 <= col < len(grid[row])

def empty_cell(row, col, grid):
    if not within_bounds(row, col, grid):
        return True
    return grid[row][col] == ' '

def new_direction(current_row, current_col, current_direction, grid):
    dirs = [(0,1),(1,0),(0,-1),(-1,0)]
    for dr, dc in dirs:
        if (dr, dc) == (-current_direction[0], -current_direction[1]):
            continue
        new_row = current_row + dr
        new_col = current_col + dc
        if not empty_cell(new_row, new_col, grid):
            return dr, dc


if __name__ == "__main__":
    with open("2017\day19\input.txt", "r") as file:
       diagram_content = file.read()
    result = path_follower(diagram_content)
    print("Collected letters:", result[0], "Steps taken:", result[1])

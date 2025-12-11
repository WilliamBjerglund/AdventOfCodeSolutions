"""
Day 10: Factory
"""

import re
import numpy as np
from scipy.optimize import milp, LinearConstraint, Bounds

# fmt: off
def parsing_machine(line):
    # light pattern
    lights = re.search(r'\[([.#]+)\]', line).group(1)
    # make it bits
    target = sum(1 << i for i, ch in enumerate(lights) if ch == "#")

    # button patterns e.g. (0, 2, 3)
    buttons = []
    for btn in re.findall(r'\(([0-9,]+)\)', line):
        # again make indicies into bitmask for botton
        buttons.append(sum(1 << int(i) for i in btn.split(",")))
    
    return target, buttons

# fmt: off
def find_minimum_button_presses_to_target(target, buttons):
    # if state is all off no presses are needed
    if target == 0:
        return 0
    
    minimum_press_count = float("inf")
    # We check if each button is inside subset and make subset
    for subset_mask in range(1, 1 << len(buttons)):
        resulting_state = 0
        press_count = 0
        for button_index in range(len(buttons)):
            if subset_mask & (1 << button_index):
                resulting_state ^= buttons[button_index]
                press_count += 1
        
        # we say if subset gets the target update minimum 
        if resulting_state == target:
            minimum_press_count = min(minimum_press_count, press_count)
    
    return minimum_press_count if minimum_press_count != float("inf") else -1

# fmt: off
def solve_all_button_presses_to_target(input_text):
    total = 0
    for line in input_text.strip().split("\n"):
        target, buttons = parsing_machine(line)
        total += find_minimum_button_presses_to_target(target, buttons)
    return total

"""
Part 2: BFS
"""
# fmt: off
def solve_all_joltage_presses(input_text):
    total = 0
    for line in input_text.strip().split("\n"):
        # Parse target joltage levels from the input {3, 5, 6, 7, 9, 11 etc..}
        target_joltage = list(map(int, re.search(r'\{([0-9,]+)\}', line).group(1).split(",")))
        
        # now we want to parse the buttons as a list of counter indices that they actually affect.
        buttons = [[int(i) for i in btn.split(",")] for btn in re.findall(r'\(([0-9,]+)\)', line)]

        # make a contraint matrix where each row should be a counter (joltage index)
        # columns should be a button and entry is 1 if the button affect the specific counter.
        constraint_matrix = np.array([[1 if counter_index in button else 0 for button in buttons] for counter_index in range(len(target_joltage))], dtype=float)


        # now we actually solve this problem using MILP where we try to minimize the sum of button presses needed to reach the target.
        optimization_result = milp(
            c=np.ones(len(buttons)),  # the objective here is to minimize button presses
            constraints=LinearConstraint(constraint_matrix, target_joltage, target_joltage),  # counter should be target
            integrality=np.ones(len(buttons)),  # must be integer
            bounds=Bounds(0, np.inf)  # no button press can be 0
        )

        total += int(round(optimization_result.fun)) # today we learned .fun should just give final result not all steps....
    return total


if __name__== "__main__":
    with open("2025/day10/input.txt") as f:
        data = f.read()
    result = solve_all_button_presses_to_target(data)
    result2 = solve_all_joltage_presses(data)
    print(f"part 1 result is: {result}, part 2 result is: {result2} ")

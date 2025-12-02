"""
Day 1: Compute the password based on dial rotations



part 1: Count how many times the dial lands on 0

- the dial shows only numbers 0-99 on a circular pattern ( it wrpas around from 99 to 0 and vice versa)
- the dial starts at position 50

- We need to read the input and check for rotation with L meaning (toward lower numbers) and R meaning (toward higher numbers)
- we need to keep track of rotations which will be the "number" after the L or R character
- as there is 100 numbers on the dial we can use modulo 100 to wrap around the dial

Answer: count how many times the dial lands on 0 after each rotation

"""


def compute_pass(input: str) -> int:
    """Compute the password based on the dial rotations"""
    pos = 50
    zero_count = 0

    for line in input.splitlines():  # Iterate through each line of the input
        line = line.strip()
        if not line:  # safety
            continue
        direction = line[0]  # Get the direction character
        steps = int(line[1:])  # Extract the number of steps
        if direction == "R":
            pos = (pos + steps) % 100
        elif direction == "L":
            pos = (pos - steps) % 100
        else:
            raise ValueError(f"Invalid direction: {direction}")  # safety
        if pos == 0:  # Check if the dial is at position 0
            zero_count += 1

    return zero_count  # Return the total count of times the dial was at position 0


"""Part 2: Compute the password on all dial clicks (method 0x434.....)"""


def compute_pass_all_clicks(input: str) -> int:
    pos = 50
    zero_count = 0

    for line in input.splitlines():
        line = line.strip()
        if not line:  # safety
            continue
        direction = line[0]
        steps = int(line[1:])  # Extract the number of steps
        if direction == "R":
            steps_delta = 1
        elif direction == "L":
            steps_delta = -1
        else:
            raise ValueError(f"Invalid direction: {direction}")  # safety
        for _ in range(steps):  # Move one step at a time
            pos = (pos + steps_delta) % 100  # Wrap around the dial
            if pos == 0:
                zero_count += 1
    return zero_count


""" The most optimized way i could find to solve this after coming back and spending too long because of a friend was to track lines in stead of steps"""
""" This way we get O(n) where n is number of lines instead of O(m) where m is number of steps"""


def optimized_all_clicks(input: str) -> int:
    pos = 50
    zero_count = 0

    for line in input.splitlines():
        line = line.strip()
        if not line:
            continue

        direction = line[0]
        steps = int(line[1:])

        if direction == "R":
            first_hit = (-pos) % 100  # == (100 - pos) % 100
            if first_hit == 0:
                first_hit = 100

            if steps >= first_hit:
                zero_count += 1 + (steps - first_hit) // 100
            pos = (pos + steps) % 100

        elif direction == "L":
            first_hit = pos % 100
            if first_hit == 0:
                first_hit = 100

            if steps >= first_hit:
                zero_count += 1 + (steps - first_hit) // 100

            pos = (pos - steps) % 100

        else:
            raise ValueError(f"Invalid direction: {direction}")

    return zero_count


if __name__ == "__main__":
    with open("2025/day1/input.txt") as f:
        input_data = f.read()

    result = compute_pass(input_data)
    result_all_clicks = compute_pass_all_clicks(input_data)
    result_optimized = optimized_all_clicks(input_data)

    print(
        f"The password is: {result}, counting all clicks: {result_all_clicks}, optimized result: {result_optimized}"
    )

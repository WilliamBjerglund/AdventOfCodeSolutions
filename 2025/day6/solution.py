"""
Day 6: Trash Compactor:
In this challenge we are told we have a huge "grid" of math problems that the cephalopods need help solving.
Their math is much like ours, but they write their numbers vertically so example:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

Would be interpreted as:
    123 * 45 * 6 = 33210
    328 + 64 + 98 = 490
    51 * 387 * 215 = 4243455
    64 + 23 + 314 = 401

Once we solve each column we then need to sum all the results together to get the final answer.
"""

# fmt: off
def Grand_Total_Math_Homework(grid: list[str]) -> int:
    rows = [line.split() for line in grid if line.strip()]
    *num_rows, ops = rows

    def prod(nums):
        r = 1
        for n in nums:
            r *= n
        return r

    total = 0
    for col, op in zip(zip(*num_rows), ops):
        nums = map(int, col)
        total += sum(nums) if op == "+" else prod(nums)
    return total


def solve(data):
    total = 0
    numbers = []

    width = len(data[-1])

    operator = ""

    for col in range(width - 1, -1, -1):
        if data[-1][col] != " ":
            operator = data[-1][col]

        new_number = "".join(
            data[row][col]
            for row in range(len(data) - 1)
            if data[row][col] != " "
        )

        if new_number == "":
            if numbers:

                total += eval(operator.join(numbers))
                numbers.clear()
        else:
            numbers.append(new_number)

    if numbers:
        total += eval(operator.join(numbers))

    return total


if __name__ == "__main__":
    with open("2025/day6/input.txt", "r") as f:
        input_data = f.read().strip().splitlines()
    with open("2025/day6/test.txt", "r") as f:
        test_data = f.read().strip().splitlines()
    result = Grand_Total_Math_Homework(input_data)
    result2 = solve(test_data)
    print(f"The sum of the mathhomework part 1 is: {result}")
    print(f"The sum of the mathhomework part 2 is: {result2}")

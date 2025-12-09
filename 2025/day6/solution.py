"""
Day 6: Trash Compactor:
"""

# fmt: off
def Grand_Total_Math_Homework(grid: list[str]) -> int:
    """ i.e. forgot about eval could have used for a more concise solution """
    parsed_rows = [line.split() for line in grid if line.strip()]
    *value_rows, operator_row = parsed_rows  # separate last row as operators

    def multiply_all(numbers): # helper function to calculate product of numbers
        product = 1
        for number in numbers:
            product *= number
        return product

    grand_total = 0
    for column_values, operator in zip(zip(*value_rows), operator_row): # iterate over columns and corresponding operators
        integer_values = map(int, column_values)
        grand_total += sum(integer_values) if operator == "+" else multiply_all(integer_values)
    return grand_total

# fmt: off
def Grand_Total_Math_homework_part2(data):
    total = 0
    current_numbers = []
    # Find the maximum width (number of columns) based on the row with the most characters
    max_column_width = max(len(row) for row in data)
    current_operator = "" 

    for col_index in range(max_column_width):
        if col_index < len(data[-1]) and data[-1][col_index] != " ":  # out of bounds safety
            current_operator = data[-1][col_index]  # Get the operator from the last row

        # Build a new number by taking non-whitespace characters from this column
        extracted_number = "".join(
            data[row_index][col_index]
            for row_index in range(len(data) - 1)
            if col_index < len(data[row_index]) and data[row_index][col_index] != " "
        )

        if extracted_number == "":  # If no number was formed, evaluate the current expression
            if current_numbers:
                total += eval(current_operator.join(current_numbers))
                current_numbers.clear()
        else:
            current_numbers.append(extracted_number)  # Add the new number to the list

    if current_numbers:  # After the loop ends, we might still have some numbers to process
        total += eval(current_operator.join(current_numbers))

    return total


if __name__ == "__main__":
    with open("2025/day6/input.txt", "r") as f:
        input_data = f.read().strip().splitlines()
    result = Grand_Total_Math_Homework(input_data)
    result2 = Grand_Total_Math_homework_part2(input_data)
    print(f"The sum of the mathhomework part 1 is: {result}, The sum of the mathhomework part 2 is: {result2}")

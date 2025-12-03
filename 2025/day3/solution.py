"""
Find the maximum joltage possible from each bank what is the total output joltage

in a string like "987654321111111" for example 98 would be the highest joltage as we cannot sort the numbers.
We have to the numbers as is look through each string and find the highest two digit number we can make.


"""

def find_max_joltage(input: str) -> int:
    """ 
    Here we find max joltage from input by checking each possible two digit combination.
    if the combination is higher than the current max_joltage we update it.
    """
    max_joltage = 0
    for i in range(len(input)):
        a = int(input[i])
        for j in range(i + 1, len(input)):
            b = int(input[j])
            joltage = a * 10 + b
            if joltage > max_joltage:
                max_joltage = joltage
    return max_joltage
        

"""
Part 2:
Now we want to find the same maximum again but this time we want to find 12 digits instead of 2.
thus in the example "987654321111111" we wound find 987654321111 as the maximum joltage.
"""
def greedy_max_joltage_12_digits(input: str, k: int = 12) -> int:
    input = input.strip()
    length = len(input) # total number of digits in the input

    if length <= k: # if fewer digits than k just return the input as integer
        return int(input)
    result_digits = []
    search_start_index = 0 # start index for searching maximum digit

    for position in range(k): 
        digits_left = k - position # digits left to find
        search_end_index = length - digits_left + 1 # end index for searching maximum digit

        window = input[search_start_index:search_end_index] # current window to search maximum digit
        max_digit = max(window)

        chosen_index = window.index(max_digit) + search_start_index # index of max digit in orginal input
        result_digits.append(max_digit) # append max digit to result
        search_start_index = chosen_index + 1 # move start index to right of chosen digit
    return int(''.join(result_digits))


def max_possible_joltage_all_strings(input_text: str, mode: int = 2) -> int:
    """ Find the total maximum joltage from all strings in the input text. """
    total = 0
    for line in input_text.splitlines():
        line = line.strip()
        if not line:
            continue
        if mode == 2:
            total += find_max_joltage(line)
        elif mode == 12:
            total += greedy_max_joltage_12_digits(line, k=12)
        else:
            raise ValueError("Invalid mode selected.") # safety
        
    return total


if __name__ == "__main__":
    with open("2025/day3/input.txt") as f:
        input_data = f.read().strip()
    result = max_possible_joltage_all_strings(input_data, mode=2)
    result_2 = max_possible_joltage_all_strings(input_data, mode=12)
    print("\n", f"Maximum joltage part 1: {result}", f"Maximum joltage part 2: {result_2}")



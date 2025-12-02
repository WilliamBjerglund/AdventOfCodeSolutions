"""
Day 2: Gift Shop — find invalid IDs

An ID is invalid if:
- the ID's digits consist of some sequence repeated twice (e.g., 55, 1212, 123123).
- IDs do not have leading zeroes (e.g., 0101 is not a valid ID). 

Input format:
- A single line containing ranges separated by commas.
- Each range is "start-end" where start and end are numeric IDs (e.g., 11-22, 998-1012).

Task:
- Find all invalid IDs that appear in the given ranges and sum them.
"""
def is_invalid_id(id_str):
    length = len(id_str)
    # ID must have even length to be two repeated halves
    if length % 2 != 0:
        return False
    half = length // 2
    return id_str[:half] == id_str[half:]

def sum_invalid_ids(ranges):
    invalid_sum = 0
    for r in ranges:
        start, end = map(int, r.split("-"))
        for id_num in range(start, end + 1):
            id_str = str(id_num)
            if is_invalid_id(id_str):
                invalid_sum += id_num
    return invalid_sum

""" 
Part 2 Clerk discovers that there are still invalid IDS:
Identifies and counts invalid IDs in a given range. An ID is invalid if it consists of a sequence of digits repeated at least twice. 

Examples of invalid IDs:
- 12341234 (1234 repeated twice)
- 111111 (1 repeated six times)

The function checks each number in the range and sums all invalid IDs.
"""
def is_invalid_id_part2(id_str):
    # Check for any repeated sequence of digits
    length = len(id_str)
    for seq_len in range(1, length // 2 + 1):
        if length % seq_len == 0:
            times = length // seq_len
            if id_str == id_str[:seq_len] * times:
                return True
    return False

def sum_invalid_ids_part2(ranges):
    invalid_sum = 0
    for r in ranges:
        start, end = map(int, r.split("-"))
        for id_num in range(start, end + 1):
            id_str = str(id_num)
            if is_invalid_id_part2(id_str):
                invalid_sum += id_num
    return invalid_sum



if __name__ == "__main__":
    with open("2025/day2/input.txt") as f:
        line = f.read().strip()
    # Parse ranges from a single comma-separated line like: "11-22,998-1012"
    ranges = [r.strip() for r in line.split(',') if r.strip()]
    result = sum_invalid_ids(ranges)
    result_part2 = sum_invalid_ids_part2(ranges)
    print("\n", result, "\n", result_part2)
    
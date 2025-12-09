"""
Day 2: Gift Shop — find invalid IDs
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
    ranges = [r.strip() for r in line.split(",") if r.strip()]
    result = sum_invalid_ids(ranges)
    result_part2 = sum_invalid_ids_part2(ranges)
    print("\n", result, "\n", result_part2)

"""
Day 5: Cafeteria 
In this challenge we are told the elves have gotten a new database.
We are told they need help extracting some information from it.

- In the database the first many lines are ranges of fresh ingredients.
- Then a blank line.
- Then many many lines of ingridients.

We want to determine how many ingredients fall within any of the given ranges and find out the amount of fresh ingredients.
"""

def count_fresh_ingredients(lines):
    fresh_ranges = []
    fresh_count = 0
    reading_ranges = True

    for line in lines:
        lines = line.strip()

        # switch from reading ranges to reading ingredients
        if lines == "":
            reading_ranges = False
            continue
        
        if reading_ranges:
            start, end = map(int, lines.split("-"))
            fresh_ranges.append((start, end))
        
        else:
            x = int(line)
            # check if any matches in the ingredients list
            for a, b in fresh_ranges:
                if a <= x <= b:
                    fresh_count += 1
                    break
    
    return fresh_count


def count_fresh_coverage(lines):
    """
    Part 2: Now we want to determine the total coverage of fresh ingredients.
    That is, how many unique ingredients are covered by the given ranges.
    """
    ranges = set() # use a set to avoid duplicates

    for line in lines: # This for loop reads the ranges
        line = line.strip()
        if line == "":
            break
        start, end = map(int, line.split("-"))
        ranges.add((start, end))

    sorted_ranges = sorted(ranges) # sort ranges by start value

    merged = [] # merged ranges
    for start, end in sorted_ranges: # This for loop merges overlapping ranges
        if not merged:
            merged.append([start, end]) # add first range
        else:
            last_start, last_end = merged[-1] # get last merged range
            if start <= last_end + 1: # check for overlap
                merged[-1][1] = max(last_end, end) # merge ranges
            else:
                merged.append([start, end]) # no overlap, add new range

    return sum(end - start + 1 for start, end in merged) # calculate total coverage
    
    


if __name__ == "__main__":
    with open("2025/day5/input.txt", "r") as f:
        lines = f.readlines()
    result = count_fresh_ingredients(lines)
    result2 = count_fresh_coverage(lines)
    print("\n", f"The result of part 1 is {result}, The result of part 2 is {result2}")

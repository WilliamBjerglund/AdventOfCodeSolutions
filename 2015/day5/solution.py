def nicestring(input):
    vowels = set("aeiou")
    forbidden = ["ab", "cd", "pq", "xy"]
    vowels_count = sum(1 for char in input if char in vowels)
    if vowels_count < 3:
        return False
    double = any(input[i] == input[i+1] for i in range(len(input)-1))
    if not double:
        return False
    contains_forbidden = any(sub in input for sub in forbidden)
    if contains_forbidden:
        return False
    return True

def nicestring2(input):
    def has_non_overlapping_pair():
        for i in range(len(input) - 1):
            pair = input[i:i+2]
            if pair in input[i+2:]:
                return True
        return False
    
    def has_repeat_with_one_between():
        for i in range(len(input) - 2):
            if input[i] == input[i+2]:
                return True
        return False
    
    return has_non_overlapping_pair() and has_repeat_with_one_between()

if __name__ == "__main__":
    with open("2015/day5/input.txt") as f:
        lines = f.read().strip().splitlines()
    
    nice_count = sum(1 for line in lines if nicestring(line))
    print("Number of nice strings:", nice_count)
    nice_count2 = sum(1 for line in lines if nicestring2(line))
    print("Number of nice strings (new rules):", nice_count2)
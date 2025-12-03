"""
Find the maximum joltage possible from each bank what is the total output joltage

in a string like "987654321111111" for example 98 would be the highest joltage as we cannot sort the numbers.
We have to the numbers as is look through each string and find the highest two digit number we can make.


"""

def find_max_joltage(input: str) -> int:
    max_joltage = 0
    for i in range (len(input) - 1):
        
print(find_max_joltage("811111111111119"))  # Example usage

input = "811111111111119" "987654321111111" "234234234234278" "818181911112111"

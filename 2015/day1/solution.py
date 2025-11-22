def SantaFloor():
    """Calculate the final floor and the position where Santa first enters the basement."""
    with open("2015/day1/input.txt") as f:
        instructions = f.read().strip()

    floor = 0
    basement_pos = None
    for i, ch in enumerate(instructions, start=1):
        if ch == "(":
            floor += 1
        elif ch == ")":
            floor -= 1

        if floor == -1 and basement_pos is None:
            basement_pos = i

    print("Final floor:", floor)
    if basement_pos is not None:
        print("First entered basement at position:", basement_pos)
    else:
        print("Never entered the basement")


SantaFloor()

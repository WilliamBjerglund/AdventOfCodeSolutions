def SantaFloor():
    """
    Day 1: but this time we want to track a 2D position and for each Charecter ^v>< we move accordingly.
    We want to count how many houses receive at least one present.
    """
    with open("2015/day3/input.txt") as f:
        instructions = f.read().strip()

    house = {(0, 0)}
    x, y = 0, 0
    for ch in instructions:
        if ch == "^":
            y += 1
        elif ch == "v":
            y -= 1
        elif ch == ">":
            x += 1
        elif ch == "<":
            x -= 1
        house.add((x, y))
    print("Number of houses that receive at least one present:", len(house))
    # Part 2 with (Another Santa starts same location and they take turns moving)
    house = {(0, 0)}
    SantaX, SantaY = 0, 0
    RoboX, RoboY = 0, 0
    for ch in instructions[::2]:
        if ch == "^":
            SantaY += 1
        elif ch == "v":
            SantaY -= 1
        elif ch == ">":
            SantaX += 1
        elif ch == "<":
            SantaX -= 1
        house.add((SantaX, SantaY))
    for ch in instructions[1::2]:
        if ch == "^":
            RoboY += 1
        elif ch == "v":
            RoboY -= 1
        elif ch == ">":
            RoboX += 1
        elif ch == "<":
            RoboX -= 1
        house.add((RoboX, RoboY))
    print(
        "Number of houses that receive at least one present (with Robo-Santa):",
        len(house),
    )


SantaFloor()

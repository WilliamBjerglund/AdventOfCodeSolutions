def wrapping__paper_needed(dimensions):
    """Calculate total wrapping paper and ribbon needed for given box dimensions."""
    with open("2015/day2/input.txt") as f:
        dimensions = f.read().strip().split("\n")

    total_paper = 0
    total_ribbon = 0

    for dimension in dimensions:
        L, W, H = map(int, dimension.split("x"))
        side1 = L * W
        side2 = W * H
        side3 = H * L
        paper_area = 2 * side1 + 2 * side2 + 2 * side3
        extra_area = min(side1, side2, side3)
        total_paper += extra_area + paper_area
        # part 2
        ribbon_wrap = 2 * sum(
            sorted([L, W, H])[:2]
        )  # smallest perimeter of any one face
        ribbon_bow = L * W * H  # volume
        total_ribbon += ribbon_wrap + ribbon_bow
    return f"Total wrapping paper needed: {total_paper}, and Total ribbon needed: {total_ribbon}"


print(wrapping__paper_needed("2015/day2/input.txt"))

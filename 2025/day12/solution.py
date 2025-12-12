"""
Day 12: Christmas Tree Farm:

not okay, if you do with example data it wont work, real data however will.
we spent too long making a backtracking solution but the problem is much simpler.
"""


def parse_input(text):
    sections = text.strip().split("\n\n")

    # Parse shapes - all sections except the last are shapes
    shapes = {}
    for block in sections[:-1]:
        lines = block.split("\n")
        shape_id = int(lines[0].rstrip(":"))
        # Count '#' characters in the shape
        area = sum(line.count("#") for line in lines[1:])
        shapes[shape_id] = area

    # Parse regions - last section
    regions = []
    for line in sections[-1].split("\n"):
        dimension_part, counts_part = line.split(":")
        width, height = map(int, dimension_part.split("x"))
        shape_counts = list(map(int, counts_part.split()))
        regions.append((width, height, shape_counts))

    return shapes, regions


def count_regions_with_enough_area(shapes, regions):
    valid_count = 0

    for width, height, shape_counts in regions:
        # Calculate total area needed for all shapes
        total_needed = sum(shapes[i] * count for i, count in enumerate(shape_counts))

        # Check if region can fit all shapes
        available_area = width * height
        if available_area >= total_needed:
            valid_count += 1

    return valid_count


if __name__ == "__main__":
    with open("2025/day12/input.txt", "r") as file:
        data = file.read().strip()
        shapes, regions = parse_input(data)
        print(count_regions_with_enough_area(shapes, regions))

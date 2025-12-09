"""
Day 9: Movie Theater

Part 1:
We are asked to look at a "2d map/grid" we are given a set of coordinates to red tiles.
we want to look at these and figure out if they form this grid what is the largest rectangle we can make.

- my approach is to first check every pair of red tiles, then if they form a rectangle calculate the area and continue until we reach the end always appending the max area.
"""

import numpy as np
from itertools import combinations

# fmt: off
def find_maximum_rectangle_area(red_tiles):
    red_tiles_set = set(red_tiles) # Make set for the case that duplicates should be present
    maximum_area = 0 # what we want to find var

    # Iterate over all pairs of tiles, using min/max to define the rectangle's corners
    for i in range(len(red_tiles)): 
        x1, y1 = red_tiles[i]  
        for j in range(i + 1, len(red_tiles)): 
            x2, y2 = red_tiles[j]

            min_x, max_x = min(x1, x2), max(x1, x2)
            min_y, max_y = min(y1, y2), max(y1, y2)
            bottom_left_corner = (min_x, min_y)
            top_left_corner = (min_x, max_y)
            bottom_right_corner = (max_x, min_y)
            top_right_corner = (max_x, max_y)

            # Case 1: If bottom left and Top right corners present in set
            if (bottom_left_corner in red_tiles_set and top_right_corner in red_tiles_set) and {(x1, y1), (x2, y2)} == {bottom_left_corner, top_right_corner}: 
                area = (max_x - min_x + 1) * (max_y - min_y + 1) # calculate area 
                maximum_area = max(maximum_area, area)

            # Case 2: if Top-left and bottom-right corners present in set
            elif (top_left_corner in red_tiles_set and bottom_right_corner in red_tiles_set) and {(x1, y1), (x2, y2)} == {top_left_corner, bottom_right_corner}:
                area = (max_x - min_x + 1) * (max_y - min_y + 1) # calculate area
                maximum_area = max(maximum_area, area)

    return maximum_area


def find_maximum_rectangle_area_part2(red_tiles):
    red_set = set(red_tiles)
    
    # Build polygon and track x-ranges for each y
    n = len(red_tiles)
    x_ranges_by_y = {}
    
    # First, mark all red and edge points
    for i in range(n):
        x1, y1 = red_tiles[i]
        x2, y2 = red_tiles[(i + 1) % n]
        
        # Mark red point
        if y1 not in x_ranges_by_y:
            x_ranges_by_y[y1] = []
        x_ranges_by_y[y1].append((x1, x1))
        
        # Mark edge points
        if x1 == x2:  # Vertical
            y_start, y_end = min(y1, y2), max(y1, y2)
            for y in range(y_start, y_end + 1):
                if y not in x_ranges_by_y:
                    x_ranges_by_y[y] = []
                x_ranges_by_y[y].append((x1, x1))
        else:  # Horizontal
            x_start, x_end = min(x1, x2), max(x1, x2)
            if y1 not in x_ranges_by_y:
                x_ranges_by_y[y1] = []
            x_ranges_by_y[y1].append((x_start, x_end))
    
    # Merge ranges for each y
    for y in x_ranges_by_y:
        ranges = sorted(x_ranges_by_y[y])
        merged = []
        current_start, current_end = ranges[0]
        
        for start, end in ranges[1:]:
            if start <= current_end + 1:
                current_end = max(current_end, end)
            else:
                merged.append((current_start, current_end))
                current_start, current_end = start, end
        
        merged.append((current_start, current_end))
        x_ranges_by_y[y] = merged
    
    def point_is_green_or_red(x, y):
        """Check if point (x,y) is in green/red area"""
        if y not in x_ranges_by_y:
            return False
        for start, end in x_ranges_by_y[y]:
            if start <= x <= end:
                return True
        return False
    
    def rectangle_is_valid(x1, y1, x2, y2):
        """Check if all points in rectangle are green/red"""
        left, right = min(x1, x2), max(x1, x2)
        bottom, top = min(y1, y2), max(y1, y2)
        
        for y in range(bottom, top + 1):
            if y not in x_ranges_by_y:
                return False
            # Check if entire horizontal line [left, right] is contained
            ranges = x_ranges_by_y[y]
            # We need left..right to be within a single continuous range
            found = False
            for start, end in ranges:
                if start <= left and right <= end:
                    found = True
                    break
            if not found:
                return False
        
        return True
    
    # Check all red point pairs
    max_area = 0
    red_list = list(red_set)
    
    for i in range(len(red_list)):
        x1, y1 = red_list[i]
        for j in range(i + 1, len(red_list)):
            x2, y2 = red_list[j]
            
            left, right = min(x1, x2), max(x1, x2)
            bottom, top = min(y1, y2), max(y1, y2)
            
            # Must be opposite corners
            is_diagonal1 = ((x1, y1) == (left, bottom) and (x2, y2) == (right, top))
            is_diagonal2 = ((x1, y1) == (left, top) and (x2, y2) == (right, bottom))
            
            if not (is_diagonal1 or is_diagonal2):
                continue
            
            if rectangle_is_valid(x1, y1, x2, y2):
                area = (right - left + 1) * (top - bottom + 1)
                max_area = max(max_area, area)
    
    return max_area

if __name__ == "__main__":
    with open("2025/day9/input.txt") as f:
        red_tiles = [tuple(map(int, line.split(','))) for line in f]
    result = find_maximum_rectangle_area(red_tiles)
    result2 = find_maximum_rectangle_area_part2(red_tiles)
    print(f"Maximum rectangle area: {result}, maximum area of rectangle using red and green tiles: {result2}")

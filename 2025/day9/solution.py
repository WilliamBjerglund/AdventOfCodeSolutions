"""
Day 9: Movie Theater
SHOUTOUT TO https://www.youtube.com/watch?v=RyLuE5xFLxw
"""

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


"""
Multiple Color ray casting/scanlines bs welcome to hell part 2
"""

def find_maximum_rectangle_area_part2(red_tiles):
    red_points_coords = [(x, y) for x, y in red_tiles]
    red_set = set(red_points_coords)
    n = len(red_points_coords)
    
    edges = [] # Build edgs conneting all consecutive points and wrap from last to first
    all_y_values = set() # Collect all y values covered by edges
    
    for i in range(n):
        x1, y1 = red_points_coords[i]
        x2, y2 = red_points_coords[(i + 1) % n]
        edges.append(((x1, y1), (x2, y2)))
        
        # add all y values covered by this edge
        y_start, y_end = min(y1, y2), max(y1, y2)
        for y in range(y_start, y_end + 1):
            all_y_values.add(y)
    
    
    all_y_values = sorted(all_y_values)
    if not all_y_values:
        return 0
    
    x_ranges_by_y = {} # Dictionary to store x ranges for each y in the polygon at specific row.
    
    for y in all_y_values: # scanline algorithm for y
        crossings = []
        
        for (x1, y1), (x2, y2) in edges:
            y_min, y_max = min(y1, y2), max(y1, y2)
            
            if y_min < y <= y_max:
                if x1 == x2:  # Vertical edge
                    crossings.append(x1) 
            elif y == y1 == y2:  # Horizontal edge at this y for the scanline
                x_min, x_max = min(x1, x2), max(x1, x2)
                if y not in x_ranges_by_y:
                    x_ranges_by_y[y] = []
                x_ranges_by_y[y].append((x_min, x_max)) # Add horizontal edge range to x_ranges_by_y (interior / boundary)
        
        # use vertical edges to find inside ranges (we want to form interior spans)
        if crossings:
            crossings.sort()
            
            # pair crossings where each pair is an inside interval
            inside_ranges = []
            for k in range(0, len(crossings), 2):
                if k + 1 < len(crossings):
                    inside_ranges.append((crossings[k], crossings[k + 1]))
            
            # merge with existing horizontal ranges
            if inside_ranges:
                if y not in x_ranges_by_y:
                    x_ranges_by_y[y] = []
                x_ranges_by_y[y].extend(inside_ranges)
    
    # Merge overlapping or adjacent x-ranges for each y
    merged_ranges_by_y = {}
    for y in x_ranges_by_y:
        ranges = x_ranges_by_y[y]
        if not ranges:
            continue
            
        ranges.sort(key=lambda r: r[0]) # sort according to start pos
        
        merged = []
        current_start, current_end = ranges[0]
        
        for start, end in ranges[1:]:
            # merge into single continuous interval (integer coords)
            if start <= current_end + 1:
                current_end = max(current_end, end)
            else:
                merged.append((current_start, current_end))
                current_start, current_end = start, end
        
        # append the final active interval
        merged.append((current_start, current_end))
        merged_ranges_by_y[y] = merged
    
    
    # sorted list of unique red points.
    max_area = 0
    red_points_list = list(red_set)
    red_points_list.sort()
    
    # go through all pairs as potential opposite corners in a rectangle.
    for i in range(len(red_points_list)):
        x1, y1 = red_points_list[i]
        
        for j in range(i + 1, len(red_points_list)):
            x2, y2 = red_points_list[j]
            
            # Must be different in both x and y to be opposite corners
            if x1 == x2 or y1 == y2:
                continue
            
            # Determine rectangle coordinates
            left_x, right_x = min(x1, x2), max(x1, x2)
            bottom_y, top_y = min(y1, y2), max(y1, y2)
            
            # Ensure this pair actually corresponds to a diagonal.
            is_bottom_left_top_right = (x1 == left_x and y1 == bottom_y and 
                                       x2 == right_x and y2 == top_y)
            is_top_left_bottom_right = (x1 == left_x and y1 == top_y and 
                                       x2 == right_x and y2 == bottom_y)
            
            if not (is_bottom_left_top_right or is_top_left_bottom_right):
                continue
            
            # Check if the entire rectangle is inside the polygon
            rectangle_valid = True
            
            # Check each row in the rectangle
            for y in range(bottom_y, top_y + 1):
                if y not in merged_ranges_by_y:
                    rectangle_valid = False
                    break
                
                # Check if [left_x, right_x] is contained in one of the ranges at this y
                row_contains_rectangle = False
                for range_start, range_end in merged_ranges_by_y[y]:
                    if range_start <= left_x and right_x <= range_end:
                        row_contains_rectangle = True
                        break
                
                if not row_contains_rectangle:
                    rectangle_valid = False
                    break
            
            if rectangle_valid:
                width = right_x - left_x + 1
                height = top_y - bottom_y + 1
                area = width * height
                max_area = max(max_area, area)
    
    return max_area


if __name__ == "__main__":
    with open("2025/day9/david.txt") as f:
        red_tiles = [tuple(map(int, line.split(','))) for line in f]
    result = find_maximum_rectangle_area(red_tiles)
    result2 = find_maximum_rectangle_area_part2(red_tiles)
    print(f"Maximum rectangle area: {result}, maximum area of rectangle using red and green tiles: {result2}")

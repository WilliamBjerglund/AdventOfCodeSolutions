"""
Day 11: Reactor
"""

# fmt: off
def count_all_paths_from_start_to_end(device_graph, current_device, destination_device, already_visited_devices=None):
    if already_visited_devices is None:
        already_visited_devices = set()
    
    # base case
    if current_device == destination_device:
        return 1 # found a path to the destination
    
    # visited this device already
    already_visited_devices.add(current_device)

    # recurvise shit
    total_number_of_paths = 0
    if current_device in device_graph:
        for connected_output_device in device_graph[current_device]:
            if connected_output_device not in already_visited_devices:
                paths_from_current_branch = count_all_paths_from_start_to_end(
                    device_graph,
                    connected_output_device,
                    destination_device,
                    already_visited_devices
                )
                total_number_of_paths += paths_from_current_branch
    
    # we remove this device from the visited list that way others can use it
    already_visited_devices.remove(current_device)  
    return total_number_of_paths

# fmt: off
def parse_devices_from_input(input_text):
    device_connection_graph = {}
    for input_line in input_text.strip().split("\n"):
        if ":" in input_line:
            source_device_name, connected_devices_string = input_line.split(':')
            source_device_name = source_device_name.strip()
            list_of_connected_devices = connected_devices_string.strip().split()
            device_connection_graph[source_device_name] = list_of_connected_devices
    return device_connection_graph

""" Part 2: We now just want to check if dac and fft are both visited in the path from svr to out """

# fmt: off
from functools import cache

def count_paths_visiting_required_devices(device_graph, start_device:str, end_device:str, required_devices):
    required_devices_frozen = frozenset(required_devices)
    
    @cache
    def counts_paths_from_state(current_device, visited_required_devices):
        # base case
        if current_device == end_device:
            # only count this path if all required devices have been visited
            if visited_required_devices == required_devices_frozen:
                return 1  
            else:
                return 0  
        
        # check if current device is a required device
        new_visited_required = visited_required_devices
        if current_device in required_devices:  
            new_visited_required = visited_required_devices | frozenset({current_device})
        
        # explore all connected devices
        total_valid_paths = 0
        if current_device in device_graph:
            for next_device in device_graph[current_device]:
                paths_visiting_dac_and_fft = counts_paths_from_state(next_device, new_visited_required)
                total_valid_paths += paths_visiting_dac_and_fft
        
        return total_valid_paths
    
    # initial call from start device with no required devices visited
    return counts_paths_from_state(start_device, frozenset())
    
if __name__ == "__main__":
    with open("day11/input.txt", "r") as file:
        input_data = file.read()
    
    parsed_device_graph = parse_devices_from_input(input_data)
    number_of_different_paths = count_all_paths_from_start_to_end(parsed_device_graph, 'you', 'out')
    paths_visiting_dac_and_fft = count_paths_visiting_required_devices(parsed_device_graph, 'svr', 'out', {'dac', 'fft'})
    print("\nNumber of different paths from 'you' to 'out':", number_of_different_paths)
    print("\n Parts from svr to out via both dac and fft:", paths_visiting_dac_and_fft)

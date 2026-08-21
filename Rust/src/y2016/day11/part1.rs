// ! --- Day 11: Radioisotope Thermoelectric Generators ---
/*
Holy fucking fuck fuck,  this is the worst one of a lifetime....
Bruteforce is good my friends.

*/
use std::collections::{HashSet, VecDeque};

type Item = [u8; 2];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    elevator: usize,
    floors: [Vec<Item>; 4],
}

/// parse input
///  we take things and abbreviate so strontium generator is SG and strontium compatible microcchip is SM and so  on...
fn parse_input(input: &str) -> State {
    let mut state = State {
        elevator: 0,
        floors: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
    };

    let mut floor_index = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // remove  all , and .
        let cleaned = line.replace(",", "").replace(".", "");
        let words: Vec<&str> = cleaned.split_whitespace().collect();

        // look at  all  words on current floor
        for word_index in 0..words.len() {
            //  if the word is  "generator"  then  we know we want  the format "<element>G" so we do that
            if words[word_index] == "generator" {
                //  the element name is the word before it
                let element_name = words[word_index - 1];

                //  we want the first letter of the element name and make it uppercase
                let element_letter = element_name.as_bytes()[0].to_ascii_uppercase();

                //  create the generator item
                let generator = [element_letter, b'G'];

                state.floors[floor_index].push(generator);
            }

            // now do the same but for microchips
            if words[word_index] == "microchip" && word_index > 0 {
                let element_name = words[word_index - 1];

                let element_letter = element_name.as_bytes()[0].to_ascii_uppercase();

                let microchip = [element_letter, b'M'];

                state.floors[floor_index].push(microchip);
            }
        }

        floor_index += 1;
    }

    for floor_index in 0..4 {
        state.floors[floor_index].sort_unstable();
    }

    return state;
}

/// check if the floor state is  safe meaning no microchip is fried.
fn floor_safe(items_present: &[Item]) -> bool {
    let mut floor_has_generator = false;

    // check if there is a generator on the floor
    for item in items_present {
        if item[1] == b'G' {
            floor_has_generator = true;
            break;
        }
    }

    // if there is no  generator on the floor, then all microchips are safe
    if !floor_has_generator {
        return true;
    }

    // if there is a generator  then all microchips must have their corresponding generator on the same floor to be safe
    for microchip in items_present {
        if microchip[1] != b'M' {
            continue;
        }

        let microchip_element = microchip[0];
        let mut microchip_safe = false;

        for generator in items_present {
            if generator[1] != b'G' {
                continue;
            }

            let generator_element = generator[0];

            if generator_element == microchip_element {
                microchip_safe = true;
                break;
            }
        }

        // there is radiation  on the floor and this  microchip does not have a generator and is therefore fried, so the floor is not safe
        if !microchip_safe {
            return false;
        }
    }

    return true;
}

///  Win condition is that all items are on the 4th floor  so that is when puzzle is finished.
fn puzzle_finished(state: &State) -> bool {
    for floor_index in 0..3 {
        if !state.floors[floor_index].is_empty() {
            return false;
        }
    }

    return true;
}

/// We check if  anything is below our elavtor cause  if its not then we should just never  go down.
fn exist_below_elavator(state: &State) -> bool {
    for floor_index in 0..state.elevator {
        if !state.floors[floor_index].is_empty() {
            return true;
        }
    }

    return false;
}

/// We are allowed to move one or two items at a time, using the elvator.
/// so we can try to move one item or two items up  or down, if the move is legal and valid we can return Some(new_state) otherwise we return None
fn attempt_moving_items(
    current_state: &State,
    destination_floor: usize,
    items_to_move: &[Item],
) -> Option<State> {
    let elevator_floor = current_state.elevator;

    // we copy our current state because i  am brute forcing this  shit
    let mut copy_state = current_state.clone();

    for item in items_to_move {
        let mut item_position = None;

        // find the  item on current elevator floor
        for item_index in 0..copy_state.floors[elevator_floor].len() {
            if copy_state.floors[elevator_floor][item_index] == *item {
                item_position = Some(item_index);
                break;
            }
        }

        let item_position = match item_position {
            Some(position) => position,
            None => return None,
        };

        // remove the item from the current floor
        let moved_item = copy_state.floors[elevator_floor].remove(item_position);

        copy_state.floors[destination_floor].push(moved_item);
    }

    // elevator also needs to move with items
    copy_state.elevator = destination_floor;

    copy_state.floors[elevator_floor].sort_unstable();
    copy_state.floors[destination_floor].sort_unstable();

    if !floor_safe(&copy_state.floors[elevator_floor]) {
        return None;
    }

    if !floor_safe(&copy_state.floors[destination_floor]) {
        return None;
    }

    return Some(copy_state);
}

/// Finally a BFS search the simplest bruteforce here.
/// it  should  garantee the  shortest path  thus minimum amount of moves eventually because eveything has a cost of 1.
fn solve_by_bfs(starting_state: State) -> Option<usize> {
    // all the states that are  waiting to be explored as (State, moves_count)
    let mut queue: VecDeque<(State, usize)> = VecDeque::new();

    // already discovered cache
    let mut discovered: HashSet<State> = HashSet::new();

    queue.push_back((starting_state.clone(), 0));

    discovered.insert(starting_state);

    let mut last_printed_moves = 0;

    while !queue.is_empty() {
        //  start  from oldest state in queue
        let queue_entry = queue.pop_front();

        let (current_state, moves_count) = match queue_entry {
            Some(entry) => entry,
            None => break,
        };

        if moves_count >= last_printed_moves + 5 {
            println!("Searching {} moves", moves_count);
            last_printed_moves = moves_count;
        }

        // check if we have reached the goal
        if puzzle_finished(&current_state) {
            return Some(moves_count);
        }

        let current_floor = current_state.elevator;

        // items the elavator currently can pick up
        let items_on_current_floor = &current_state.floors[current_floor];

        // we can move up or down  with -1 or +1
        let directions = [-1, 1];

        for index in 0..directions.len() {
            let direction = directions[index];

            // only move down if there is something there
            if direction == -1 && !exist_below_elavator(&current_state) {
                continue;
            }

            let destination_floor = current_floor as i32 + direction;

            if destination_floor < 0 || destination_floor > 3 {
                continue;
            }

            let destination_floor = destination_floor as usize;

            // Attempt to move only one item
            for first_item_index in 0..items_on_current_floor.len() {
                let first_item = items_on_current_floor[first_item_index];

                let items_to_move = first_item;

                let move_result =
                    attempt_moving_items(&current_state, destination_floor, &[items_to_move]);

                let new_state = match move_result {
                    Some(state) => state,
                    None => continue,
                };

                if discovered.insert(new_state.clone()) {
                    queue.push_back((new_state, moves_count + 1));
                }
            }

            // now attempt to move two items
            for first_item_index in 0..items_on_current_floor.len() {
                for second_item_index in first_item_index + 1..items_on_current_floor.len() {
                    let first_item = items_on_current_floor[first_item_index];
                    let second_item = items_on_current_floor[second_item_index];

                    let items_to_move = [first_item, second_item];

                    let move_result =
                        attempt_moving_items(&current_state, destination_floor, &items_to_move);

                    let new_state = match move_result {
                        Some(state) => state,
                        None => continue,
                    };

                    if discovered.insert(new_state.clone()) {
                        queue.push_back((new_state, moves_count + 1));
                    }
                }
            }
        }
    }

    return None;
}

/// Main function that runs the entire solver for part 1 and gets reused for part 2
pub(super) fn solve_extra_items(input: &str, extra_items: &[[u8; 2]]) -> usize {
    let input = input.trim();

    let mut starting_state = parse_input(input);

    // add things from part 2
    for item in extra_items {
        starting_state.floors[0].push(*item);
    }

    solve_by_bfs(starting_state).unwrap()
}

pub fn main(input: &str) -> usize {
    solve_extra_items(input, &[])
}

#[test]
fn test() {
    let input = "
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.
";

    assert_eq!(main(input), 11);
}

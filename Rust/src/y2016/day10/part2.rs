// ! --- Day 10: Balance Bots ---
// ! --- Part Two ---

/*
here we asked what we get if you multiply together the values of one chip in each of the outputs 0 1 2.
So the idea is essentially just instead of just ignoring output bins we now need to track them and handle them.
My plan here is to just copy paste part1 and add tthe new rule and handling.
*/

use std::collections::HashMap;

// Chips either go to another bot or to an output bin
#[derive(Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize),
}

// Store where high low chips go
#[derive(Clone, Copy)]
struct Rule {
    low: Destination,
    high: Destination,
}

#[rustfmt::skip]
pub fn main(input: &str) -> usize {
    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new(); // Current state (bot_id -> chips)
    let mut rules: HashMap<usize, Rule> = HashMap::new(); // instructions (rules)
    let mut ready: Vec<usize> = Vec::new(); // waiting with 2 chips
    let mut outputs: HashMap<usize, usize> = HashMap::new(); // output bins

    // parse full input at first
    for line in input.lines() { 
        let parts: Vec<&str> = line.split_whitespace().collect(); 

        // if its a value instruction we know the rule is to give the value to a bot so we can just add it to the bots hashmap
        if parts[0] == "value" {
            let value: usize = parts[1].parse().unwrap();// value 
            let bot_id: usize = parts[5].parse().unwrap(); // bot id
            bots.entry(bot_id).or_default().push(value); // add the value to the bot's list of chips
        } else if parts[0] == "bot" {
            let bot_id: usize = parts[1].parse().unwrap(); // read bot that owns rule.
            let low_id: usize = parts[6].parse().unwrap(); 
            let high_id: usize = parts[11].parse().unwrap(); // read the low and high destination ids

            let low = if parts[5] == "bot" {Destination::Bot(low_id)} else {Destination::Output(low_id)};
            
            let high = if parts[10] == "bot" {Destination::Bot(high_id)} else {Destination::Output(high_id)};

            // save bots full rule
            rules.insert(bot_id, Rule { low, high });
        }
    }

    // Now we find bots that stat with 2 chips.
    for (&bot_id, chips) in &bots {
        if chips.len() == 2 {
            ready.push(bot_id);
        }
    }


    // Now we make the while loop that will keep running until we find the bot with the 17 and 61 chips.
    while let Some(bot_id) = ready.pop() {
        let chips = bots.get_mut(&bot_id).unwrap(); 

        if chips.len() != 2 {
            continue; // skip if the bot doesn't have 2 chips
        }

        // remove chips
        let first = chips.pop().unwrap();
        let second = chips.pop().unwrap();

        // find the low and high chip
        let (low, high) = if first < second { (first, second) } else { (second, first)};

        // copy rule
        let rule = rules[&bot_id];

        // give chip will handle both bots and output bins
        give_chip(rule.low, low, &mut bots, &mut outputs, &mut ready);
        give_chip(rule.high, high, &mut bots, &mut outputs, &mut ready);
    }
    outputs[&0] * outputs[&1] * outputs[&2] // multiply the chips outred in the outputs 
}

#[rustfmt::skip]
fn give_chip(destination: Destination, chip: usize, bots: &mut HashMap<usize, Vec<usize>>, outputs: &mut HashMap<usize, usize>, ready: &mut Vec<usize>) {
    match destination {
        Destination::Bot(bot_id) => {
            let chips = bots.entry(bot_id).or_default(); // find receiving bot chip vector or make new one empty.
            chips.push(chip);

            // receiving bot should act once it has 2 chips
            if chips.len() == 2 {
                ready.push(bot_id);
            }
        }

        // Store the chip in its output bin
        Destination::Output(output_id) => {
            outputs.insert(output_id, chip); // store the chip in the output bin
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

        assert_eq!(main(input), 30);
    }
}

// ! --- Day 10: Balance Bots ---

/*
Rules Defined using the 17 and 61 chips as an example cause i need some visualization of what to do:

1. A bot waits until it has 2 chips with doing anything
2. When a bot has 2 chips it compares them to find the smaller and higher value chip.
3. Each instruction will tell each bot where to send both chips so ex. bot 2 gives the lower chip (17) to bot 1 and the higher chip (61) to bot 0.
4. A bot can give its chips to Bins instead of other bots, there is a output bin aswell as a input bin.
5. Every instruction that starts with value 17.... or something along those lines give each bot their starting chips.
6. a bot can only receive chips once it is empty so it has to give away both its chips before getting more.

Finally our goal is to just get these rules in parse the instructions use a cache to just keep track of everything and once our -
if checkblock finds the bot with the 17 and 61 chips we can return the bot number that has them and terminate the program.
*/

use std::collections::HashMap;

// Struct that stores where the bot sends its low and high chips.
#[derive(Clone, Copy)]
struct Rule {
    low_bot: Option<usize>,
    high_bot: Option<usize>,
}

#[rustfmt::skip]
pub fn main(input: &str) -> usize {
    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new(); // Current state (bot_id -> chips)
    let mut rules: HashMap<usize, Rule> = HashMap::new(); // instructions (rules)
    let mut ready: Vec<usize> = Vec::new(); // waiting with 2 chips

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
            let low_bot = if parts[5] == "bot" {Some(parts[6].parse().unwrap())} else {None}; 
            let high_bot = if parts[10] == "bot" {Some(parts[11].parse().unwrap())} else {None};

            // save bots full rule
            rules.insert(bot_id, Rule {
                    low_bot,
                    high_bot,
                },
            );
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
        let (low, high) = if first < second { (first, second) } else { (second, first) };

        // win condition
        if low == 17 && high == 61 {
            return bot_id; // found the bot with the 17 and 61 chips
        }

        // copy rule
        let rule = rules[&bot_id];

        if let Some(target_bot) = rule.low_bot {
            give_chip(target_bot, low, &mut bots, &mut ready); // send low chip if its dest is another bot otherwise output bin when None
        }

        if let Some(target_bot) = rule.high_bot {
            give_chip(target_bot, high, &mut bots, &mut ready); // send high chip if dest is another bot
        }
    }
    panic!("No bot found that compares 17 and 61 chips");
}

#[rustfmt::skip]
fn give_chip(bot_id: usize, chip: usize, bots: &mut HashMap<usize, Vec<usize>>, ready: &mut Vec<usize>) {
    let chips = bots.entry(bot_id).or_default(); // find receiving bot chip vector or make new one empty.
    chips.push(chip);

    if chips.len() == 2 {
        ready.push(bot_id);
    }
}

// Yeah i could not figure out a good way to do a test here and i am not going to bother part 2 has one.

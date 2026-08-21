// ! --- Day 12: Leonardo's Monorail ---
/*

Today seems rather simple and like a problem challenging on the basic understanding off assembly and memory management.
We are told that we remotely connect to a monorail control system.
We find that the boot sequence is a series of instructions that manipulate four registers (a, b, c, d).

It can perform 4 basic instructions:
cpy a b - so we copy the value of a into register b
inc a   - so we increase the value of register a by 1
dec a   - the opposite of inc.
jnz a b - jump to the isntruction b away provided a is not zero. both directions + and -

this is fairly straight forward, setup a simple interpreter that just takes some input and tracks the value inside.
*/

fn register_index(register: &str) -> usize {
    // map the register name to an index in the registers array
    match register {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Invalid register"),
    }
}

fn get_value(value: &str, registers: &[i32; 4]) -> i32 {
    // if the value is a number, return it, otherwise return the value in the register
    match value.parse::<i32>() {
        Ok(num) => num,
        Err(_) => registers[register_index(value)],
    }
}

pub(super) fn monorail_interpreter(input: &str, start_c_reg: i32) -> usize {
    let mut registers = [0, 0, start_c_reg, 0]; // a, b, c, d

    let instructions: Vec<&str> = input.lines().collect();

    // pointer to the current instruction
    let mut pointer = 0;

    while pointer < instructions.len() {
        let parts: Vec<&str> = instructions[pointer].split_whitespace().collect();

        let instruction = parts[0];

        match instruction {
            "cpy" => {
                let value = get_value(parts[1], &registers);
                let destination = register_index(parts[2]);

                registers[destination] = value;

                pointer += 1;
            }

            "inc" => {
                // get the register to increment
                let index = register_index(parts[1]);
                registers[index] += 1;

                pointer += 1;
            }

            "dec" => {
                // get the register to decrement
                let index = register_index(parts[1]);
                registers[index] -= 1;

                pointer += 1;
            }

            "jnz" => {
                let value = get_value(parts[1], &registers);

                if value != 0 {
                    let jump = get_value(parts[2], &registers);

                    if jump < 0 {
                        pointer -= (-jump) as usize;
                    } else {
                        pointer += jump as usize;
                    }
                } else {
                    pointer += 1;
                }
            }

            _ => {
                panic!("Invalid instruction");
            }
        }
    }

    registers[0] as usize
}

pub fn main(input: &str) -> usize {
    monorail_interpreter(input, 0)
}

#[test]
fn test() {
    let input = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    assert_eq!(monorail_interpreter(input, 0), 42);
}

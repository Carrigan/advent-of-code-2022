use std::fs;
use regex;

struct Instruction {
    count: i32,
    from: i32,
    to: i32
}

type Stacks = [Vec<char>; 10];

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let re = regex::Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();

        let captures = match re.captures(line) {
            Some(c) => c,
            None => return Err("Line formatted incorrectly")
        };

        let count = match captures[1].parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err("Could not parse entry 'count'")
        };

        let from = match captures[2].parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err("Could not parse entry 'from'")
        };

        let to = match captures[3].parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err("Could not parse entry 'to'")
        };

        Ok(Instruction { count, from, to })
    }
}

fn read_input(path: &str) -> (Stacks, Vec<Instruction>) {
    let input = fs::read_to_string(path).expect("File path must be valid");
    let mut line_iter = input.lines();
    let mut stacks: Stacks = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut instructions: Vec<Instruction> = Vec::new();

    while let Some(line) = line_iter.next() {
        if line == "" { break; }

        // Parse stacks until the line break
        let mut chars = line.chars();
        let mut i = 0;
        while let (Some(_a), Some(b), Some(_c)) = (chars.next(), chars.next(), chars.next()) {
            if b == '1' {
                break;
            }

            if b != ' ' {
                stacks[i].push(b);
            }

            i += 1;
            chars.next();
        }
    };

    // Reverse all the stacks
    for stack in &mut stacks {
        stack.reverse();
    }

    while let Some(line) = line_iter.next() {
        let instruction = match Instruction::try_from(line) {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                break
            }
        };

        instructions.push(instruction);
    }

    (stacks, instructions)
}

fn perform_moves(stacks: &mut Stacks, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let top = stacks[instruction.from as usize - 1].pop().unwrap();
            stacks[instruction.to as usize - 1].push(top);
        }
    }
}

fn perform_moves_9001(stacks: &mut Stacks, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        // Pop the elements into a temporary vec
        let mut temporary = Vec::new();

        for _ in 0..instruction.count {
            let top = stacks[instruction.from as usize - 1].pop().unwrap();
            temporary.push(top);
        }

        // Pop out of the temporary vec to destination
        let size = temporary.len();
        for _ in 0..size {
            stacks[instruction.to as usize - 1].push(temporary.pop().unwrap());
        }
    }

}

fn last_elements(stacks: &Stacks) -> Vec<char> {
    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .map(|c| *c)
        .collect()
}

fn main() {
    let (mut stacks, instructions) = read_input("input.txt");
    perform_moves(&mut stacks, &instructions);
    println!("{:?}", last_elements(&stacks));

    let (mut stacks, instructions) = read_input("input.txt");
    perform_moves_9001(&mut stacks, &instructions);
    println!("{:?}", last_elements(&stacks));
}

#[test]
fn test_part_one() {
    let (mut stacks, instructions) = read_input("test");
    assert_eq!(instructions.get(0).unwrap().count, 1);
    assert_eq!(*stacks.get(0).unwrap().get(0).unwrap(), 'Z');
    assert_eq!(*stacks.get(1).unwrap().get(0).unwrap(), 'M');
    assert_eq!(*stacks.get(2).unwrap().get(0).unwrap(), 'P');

    perform_moves(&mut stacks, &instructions);
    let lasts = last_elements(&stacks);

    assert_eq!(lasts[0], 'C');
    assert_eq!(lasts[1], 'M');
    assert_eq!(lasts[2], 'Z');
}

#[test]
fn test_part_two() {
    let (mut stacks, instructions) = read_input("test");

    perform_moves_9001(&mut stacks, &instructions);
    let lasts = last_elements(&stacks);

    assert_eq!(lasts[0], 'M');
    assert_eq!(lasts[1], 'C');
    assert_eq!(lasts[2], 'D');
}

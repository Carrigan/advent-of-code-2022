use std::fs;

type Calorie = u32;
type ElfInventory = Vec<Calorie>;

fn read_input(path: &str) -> Vec<ElfInventory> {
    let mut calories: Vec<ElfInventory> = Vec::new();

    let input = fs::read_to_string(path).expect("File path must be valid");

    let mut elf = Vec::new();
    for entry in input.lines() {
        match entry {
            "" => {
                calories.push(elf);
                elf = Vec::new();
            }
            _ => {
                elf.push(entry.parse::<Calorie>().expect("Lines must be parsable to u32"));
            }
        }
    }

    calories.push(elf);
    calories
}

fn highest_calories(elves: &Vec<ElfInventory>) -> Calorie {
    elves
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap()
}

fn highest_three_calories(elves: &Vec<ElfInventory>) -> Calorie {
    let mut calories: Vec<Calorie> = elves
        .iter()
        .map(|elf| elf.iter().sum())
        .collect();

    calories.sort();

    calories.iter().rev().take(3).sum()
}

fn main() {
    let elves = read_input("input.txt");
    println!("Part One: {}", highest_calories(&elves));
    println!("Part Two: {}", highest_three_calories(&elves));

}

#[test]
fn test_part_one() {
    let elves = read_input("test-part-1.txt");
    assert_eq!(highest_calories(&elves), 24000);
}

#[test]
fn test_part_two() {
    let elves = read_input("test-part-1.txt");
    assert_eq!(highest_three_calories(&elves), 45000);
}



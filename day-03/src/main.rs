use std::fs;

fn read_input(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("File path must be valid");

    input
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn split(pack: &str) -> (&str, &str) {
    let half = pack.len() / 2;
    (&pack[0..half], &pack[half..])
}

fn find_same(compartment_one: &str, compartment_two: &str) -> char {
    compartment_one
        .chars()
        .find(|&c| compartment_two.contains(c))
        .unwrap()
}

fn priority(item: char) -> u32 {
    match item {
        'a' ..= 'z' => <char as Into<u32>>::into(item) - 96,
        _ => <char as Into<u32>>::into(item) - 38
    }
}

fn sum_priorities(packs: &Vec<String>) -> u32 {
    packs
        .iter()
        .map(|pack| split(pack))
        .map(|(comp_one, comp_two)| find_same(comp_one, comp_two))
        .map(|same| priority(same))
        .sum()
}

fn main() {
    let packs = read_input("input");
    println!("Part one: {}", sum_priorities(&packs));
}

#[test]
fn test_part_one() {
    let (comp_one, comp_two) = split("mnDelpeR");
    assert_eq!(comp_one, "mnDe");
    assert_eq!('e', find_same(comp_one, comp_two));
    assert_eq!(priority('p'), 16);
    assert_eq!(priority('L'), 38);

    let packs = read_input("test");
    assert_eq!(sum_priorities(&packs), 157);
}

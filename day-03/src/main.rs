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

struct TakeThree<I> {
    iter: I
}

impl <'a, I> Iterator for TakeThree<I> where I: Iterator<Item=&'a String> {
    type Item = (&'a String, &'a String, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        Some((
            self.iter.next()?,
            self.iter.next()?,
            self.iter.next()?
        ))
    }
}

fn find_common_item(one: &str, two: &str, three: &str) -> char {
    for item in one.chars() {
        if two.contains(item) && three.contains(item) {
            return item;
        }
    }

    panic!()
}

fn sum_badges(packs: &Vec<String>) -> u32 {
    let iter = TakeThree { iter: packs.iter() };

    iter
        .map(|(one, two, three)| find_common_item(one, two, three))
        .map(|item| priority(item))
        .sum()
}

fn main() {
    let packs = read_input("input");
    println!("Part one: {}", sum_priorities(&packs));
    println!("Part two: {}", sum_badges(&packs));
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

#[test]
fn test_part_two() {
    let packs = read_input("test");
    assert_eq!(sum_badges(&packs), 70);
}

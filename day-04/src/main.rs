use std::fs;

type Range = (u32, u32);

fn read_line(line: &str) -> (Range, Range) {
    let mut parts = line.split(",");
    let first = parts.next().unwrap();
    let second = parts.next().unwrap();
    let mut first_parts = first.split("-");
    let mut second_parts = second.split("-");

    (
        (first_parts.next().unwrap().parse().unwrap(), first_parts.next().unwrap().parse().unwrap()),
        (second_parts.next().unwrap().parse().unwrap(), second_parts.next().unwrap().parse().unwrap()),
    )
}

fn read_input(path: &str) -> Vec<(Range, Range)> {
    let input = fs::read_to_string(path).expect("File path must be valid");

    input
        .lines()
        .map(|line| read_line(line))
        .collect()
}

fn fully_overlap(one: &Range, two: &Range) -> bool {
    if one.0 <= two.0 && one.1 >= two.1 {
        return true;
    }

    if two.0 <= one.0 && two.1 >= one.1 {
        return true;
    }

    false
}

fn count_fully_overlapping(sections: &Vec<(Range, Range)>) -> u32 {
    sections
        .iter()
        .map(|(r1, r2)| fully_overlap(r1, r2))
        .map(|overlap| if overlap { 1 } else { 0 })
        .sum()
}

fn main() {
    let sections = read_input("input");
    println!("Part one: {}", count_fully_overlapping(&sections));
}

#[test]
fn test_part_one() {
    let sections = read_input("test");
    assert_eq!(count_fully_overlapping(&sections), 2);
}

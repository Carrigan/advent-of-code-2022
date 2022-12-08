use std::fs;

enum Entry {
    Rock,
    Paper,
    Scissors
}

impl Entry {
    fn to_score(entry: &Entry) -> u32 {
        match entry {
            Entry::Rock => 1,
            Entry::Paper => 2,
            Entry::Scissors => 3
        }
    }

    fn find_matching(theirs: &Entry, pair_by: char) -> Entry {
        match theirs {
            Entry::Rock => match pair_by {
                'X' => Entry::Scissors,
                'Y' => Entry::Rock,
                _ => Entry::Paper
            },
            Entry::Paper => match pair_by {
                'X' => Entry::Rock,
                'Y' => Entry::Paper,
                _ => Entry::Scissors
            },
            Entry::Scissors => match pair_by {
                'X' => Entry::Paper,
                'Y' => Entry::Scissors,
                _ => Entry::Rock
            },
        }
    }
}

struct Game {
    mine: Entry,
    theirs: Entry
}

impl Game {
    fn from_p1_str(line: &str) -> Option<Game> {
        let theirs: Entry = match line.chars().nth(0).unwrap() {
            'A' => Entry::Rock,
            'B' => Entry::Paper,
            'C' => Entry::Scissors,
            _ => panic!()
        };

        let mine: Entry = match line.chars().nth(2).unwrap() {
            'X' => Entry::Rock,
            'Y' => Entry::Paper,
            'Z' => Entry::Scissors,
            _ => panic!()
        };

        Some(Game { mine, theirs })
    }

    fn from_p2_str(line: &str) -> Option<Game> {
        let theirs: Entry = match line.chars().nth(0).unwrap() {
            'A' => Entry::Rock,
            'B' => Entry::Paper,
            'C' => Entry::Scissors,
            _ => panic!()
        };

        let mine: Entry = Entry::find_matching(&theirs, line.chars().nth(2).unwrap());

        Some(Game { mine, theirs })
    }

    fn score(game: &Game) -> u32 {
        let base: u32 = Entry::to_score(&game.mine);

        let win_points: u32 = match (&game.mine, &game.theirs) {
            (Entry::Rock, Entry::Scissors) => 6,
            (Entry::Rock, Entry::Paper) => 0,
            (Entry::Scissors, Entry::Paper) => 6,
            (Entry::Scissors, Entry::Rock) => 0,
            (Entry::Paper, Entry::Rock) => 6,
            (Entry::Paper, Entry::Scissors) => 0,
            _ => 3
        };

        base + win_points
    }
}

fn read_input(path: &str) -> Vec<Game> {
    let input = fs::read_to_string(path).expect("File path must be valid");

    input
        .lines()
        .map(|line| Game::from_p1_str(line).unwrap())
        .collect()
}

fn read_input_part_2(path: &str) -> Vec<Game> {
    let input = fs::read_to_string(path).expect("File path must be valid");

    input
        .lines()
        .map(|line| Game::from_p2_str(line).unwrap())
        .collect()
}

fn total_score(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| Game::score(game))
        .sum()
}

fn main() {
    let games = read_input("input");
    println!("Part one: {}", total_score(&games));

    let games = read_input_part_2("input");
    println!("Part two: {}", total_score(&games));
}

#[test]
fn test_part_one() {
    let games = read_input("test");
    assert_eq!(total_score(&games), 15);
}

#[test]
fn test_part_two() {
    let games = read_input_part_2("test");
    assert_eq!(total_score(&games), 12);
}

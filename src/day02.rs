use std::{fs, collections::HashMap};

pub fn run() {
    let input = fs::read_to_string("./inputs/2.txt").unwrap();

    let part_one_result = part_one(input.as_str());
    let part_two_result = part_two(input.as_str());

    println!("Day 2 - part 1: {}", part_one_result);
    println!("Day 2 - part 2: {}", part_two_result);
}

fn part_one(input: &str) -> i32 {
    let constraints = HashMap::from([
        ("blue", 14),
        ("red", 12),
        ("green", 13)
    ]);

    input
        .lines()
        .map(|line| {
            if let Some((game_prefix, examples)) = line.strip_prefix("Game ").unwrap().split_once(": ") {
                let game_num = game_prefix.parse::<i32>().unwrap();

                for round in examples.split("; ") {
                    for example in round.split(", ") {
                        if let Some((quantity_str, colour)) = example.split_once(' ') {
                            if let Some(constraint) = constraints.get(colour) {
                                if quantity_str.parse::<i32>().unwrap() > *constraint {
                                    return 0;
                                }
                            }
                        }
                    }
                }

                return game_num;
            }

            0
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let mut powers = HashMap::from([
        ("blue", 0),
        ("red", 0),
        ("green", 0)
    ]);

    input
        .lines()
        .map(|line| {
            if let Some((_, examples)) = line.split_once(": ") {
                for round in examples.split("; ") {
                    for example in round.split(", ") {
                        if let Some((quantity_str, colour)) = example.split_once(' ') {
                            if let Some(power) = powers.get_mut(colour) {
                                let quantity = quantity_str.parse::<i32>().unwrap();
                                if quantity > *power {
                                    *power = quantity;
                                }
                            }
                        }
                    }
                }
            }

            let ans = powers.iter().map(|(_,v)| v).product::<i32>();

            powers.iter_mut().for_each(|(_, v)| *v = 0);

            ans
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        const EXPECTED: i32 = 8;

        let actual = part_one(INPUT);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn part_two_test() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        const EXPECTED: i32 = 2286;

        let actual = part_two(INPUT);

        assert_eq!(EXPECTED, actual);
    }
}

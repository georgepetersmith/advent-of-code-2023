use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/1.txt").unwrap();

    let part_one_result = part_one(input.as_str());
    let part_two_result = part_two(input.as_str());

    println!("Day 1 - part 1: {}", part_one_result);
    println!("Day 1 - part 2: {}", part_two_result);
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let digits = l.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();

            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let replacements = Vec::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let substituted_lines = input
        .lines()
        .map(|l| {
            let mut digit_chars: Vec<char> = Vec::new();
            for (i, c) in l.char_indices() {
                if !char::is_digit(c, 10) {
                    let substring: &str = l.get(i..).unwrap();
                    if let Some(&replacement) =
                        replacements.iter().find(|&&p| substring.starts_with(p.0))
                    {
                        digit_chars.push(replacement.1);
                    }
                } else {
                    digit_chars.push(c);
                }
            }

            String::from_iter(digit_chars) + "\n"
        })
        .collect::<String>();

    part_one(substituted_lines.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        const INPUT: &str = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        const EXPECTED: i32 = 142;
        let result = part_one(INPUT);

        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn part_two_test() {
        const INPUT: &str = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        const EXPECTED: i32 = 281;
        let result = part_two(INPUT);

        assert_eq!(result, EXPECTED);
    }
}

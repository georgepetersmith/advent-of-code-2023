use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/3.txt").unwrap();

    let part_one_result = part_one(input.as_str());
    // let part_two_result = part_two(input.as_str());

    println!("Day 3 - part 1: {}", part_one_result);
    // println!("Day 3 - part 2: {}", part_two_result);
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Number {
    value: i32,
    points: Vec<Point>,
}

fn part_one(input: &str) -> i32 {
    let mut symbols: Vec<Point> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut in_number = false;
        let mut number_chars: Vec<char> = Vec::new();
        let mut number_points: Vec<Point> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let is_last_char = x == line.len().saturating_sub(1);
            let is_numeric = c.is_numeric();

            if is_numeric {
                in_number = true;
                number_chars.push(c);
                number_points.push(Point { x, y });
            }

            if (is_numeric && is_last_char) || (!is_numeric && in_number) {
                let value = number_chars
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();

                let number = Number {
                    value,
                    points: number_points.to_vec(),
                };
                numbers.push(number);

                number_chars.clear();
                number_points.clear();
                in_number = false;
            }

            if !is_numeric && c != '.' {
                let point = Point { x, y };
                symbols.push(point);
            }
        }
    }

    numbers
        .iter()
        .filter(|n| {
            n.points.iter().any(|p| {
                symbols.iter().any(|s| {
                    p.x.saturating_sub(1) <= s.x
                        && s.x <= p.x + 1
                        && p.y.saturating_sub(1) <= s.y
                        && s.y <= p.y + 1
                })
            })
        })
        .map(|n| n.value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        const EXPECTED: i32 = 4361;
        const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = part_one(INPUT);

        assert_eq!(EXPECTED, result);
    }
}

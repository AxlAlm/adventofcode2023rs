use std::collections::HashMap;

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct NumberSection {
    id: String,
    number: u32,
}

#[derive(Debug, Clone)]
struct Schematics {
    number_sections: HashMap<(i32, i32), NumberSection>,
    symbols: HashMap<(i32, i32), char>,
}

fn parse_schematics(input: &str) -> Schematics {
    let mut number_sections: HashMap<(i32, i32), NumberSection> = HashMap::new();
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let mut current_number = "".to_string();
        let mut current_number_id = format!("{x_start}-{y_start}", x_start = 0, y_start = y);
        let mut current_number_xy: Vec<(i32, i32)> = vec![];

        for (x, c) in line.trim().chars().enumerate() {
            if c.is_digit(10) {
                current_number_xy.push((x as i32, y as i32));
                current_number.push(c);
                continue;
            }

            if current_number.len() != 0 {
                for xy in current_number_xy.iter() {
                    let n = current_number.parse::<u32>().unwrap();
                    number_sections.insert(
                        xy.clone(),
                        NumberSection {
                            id: current_number_id.to_string(),
                            number: n,
                        },
                    );
                }
            }

            current_number = "".to_string();
            current_number_id = format!("{x_start}-{y_start}", x_start = x, y_start = y);
            current_number_xy = vec![];

            if c == '.' {
                continue;
            }

            symbols.insert((x as i32, y as i32), c);
        }

        if current_number.len() != 0 {
            for xy in current_number_xy.iter() {
                let n = current_number.parse::<u32>().unwrap();
                number_sections.insert(
                    xy.clone(),
                    NumberSection {
                        id: current_number_id.to_string(),
                        number: n,
                    },
                );
            }
        }
    }

    return Schematics {
        number_sections: number_sections,
        symbols: symbols,
    };
}

fn get_surrounding_coordinates(x: i32, y: i32) -> Vec<(i32, i32)> {
    return vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
}

fn find_gear_sum(schematics: &Schematics) -> u32 {
    let mut sum = 0;
    for ((x, y), symbol) in schematics.symbols.iter() {
        let is_gear = symbol == &'*';
        if !is_gear {
            continue;
        }

        let possible_part_coordinates = get_surrounding_coordinates(*x, *y);

        let mut gear_part1: NumberSection = NumberSection {
            id: "".to_string(),
            number: 0,
        };

        let mut gear_part2: NumberSection = NumberSection {
            id: "".to_string(),
            number: 0,
        };

        for xy in possible_part_coordinates.iter() {
            if !schematics.number_sections.contains_key(xy) {
                continue;
            }
            let part = schematics.number_sections[xy].clone();

            if gear_part1.id.len() == 0 {
                gear_part1 = part;
                continue;
            }

            if gear_part2.id.len() == 0 && part.id != gear_part1.id {
                gear_part2 = part;
            }
        }

        let is_gear = gear_part1.id != "".to_string() && gear_part2.id != "".to_string();

        if !is_gear {
            continue;
        }

        sum += gear_part1.number * gear_part2.number;
    }

    return sum;
}

fn part2(input: &str) -> u32 {
    let schematics = parse_schematics(input);
    return find_gear_sum(&schematics);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {

        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected_result = 467835;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_2_only_symbol_between_numbers() {
        let test_input = "50*50..";
        let expected_result = 2500;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_2_single_numbers() {
        let test_input = "
            .5*...
            ...5..";
        let expected_result = 25;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_2_many_adj() {
        let test_input = "
            .*50*...
            ..=/....";
        let expected_result = 0;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_2_end() {
        let test_input = "
            .....*50
            ..=50...";
        let expected_result = 2500;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}


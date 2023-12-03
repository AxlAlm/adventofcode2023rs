fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct NumberSection {
    x_start: i32,
    x_end: i32,
    y: i32,
    number: String,
}

#[derive(Debug, Clone)]
struct Symbol {
    x: i32,
    y: i32,
    value: char,
}

#[derive(Debug, Clone)]
struct Schematics {
    number_sections: Vec<NumberSection>,
    symbols: Vec<Symbol>,
}

fn parse_schematics(input: &str) -> Schematics {
    let mut number_sections = vec![];
    let mut symbols = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut current_number_section = NumberSection {
            x_start: 0,
            x_end: 0,
            y: y as i32,
            number: "".to_string(),
        };
        for (x, c) in line.trim().chars().enumerate() {
            if c.is_digit(10) {
                if current_number_section.number == "" {
                    current_number_section.x_start = x as i32;
                    current_number_section.x_end = x as i32;
                } else {
                    current_number_section.x_end = x as i32;
                }

                current_number_section.number.push(c);
                continue;
            }

            if current_number_section.number != "" && !c.is_digit(10) {
                number_sections.push(current_number_section);
                current_number_section = NumberSection {
                    x_start: 0,
                    x_end: 0,
                    y: y as i32,
                    number: "".to_string(),
                };
            }

            let is_dot = c == '.';
            if is_dot {
                continue;
            }

            symbols.push(Symbol {
                x: x as i32,
                y: y as i32,
                value: c,
            })
        }

        if current_number_section.number != "" {
            number_sections.push(current_number_section);
        }
    }

    return Schematics {
        number_sections: number_sections,
        symbols: symbols,
    };
}

fn find_part_numbers(schematics: &Schematics) -> Vec<&NumberSection> {
    let mut part_numbers = vec![];
    for number_section in schematics.number_sections.iter() {
        for symbol in schematics.symbols.iter() {
            let is_x_adj =
                symbol.x >= number_section.x_start - 1 && symbol.x <= number_section.x_end + 1;
            let is_y_adj = symbol.y >= number_section.y - 1 && symbol.y <= number_section.y + 1;
            let is_adj = is_x_adj && is_y_adj;

            if !is_adj {
                continue;
            }

            part_numbers.push(number_section);
            break;
        }
    }

    return part_numbers;
}

fn part1(input: &str) -> u32 {
    let schematics = parse_schematics(input);
    let part_numbers = find_part_numbers(&schematics);
    let sum: u32 = part_numbers
        .into_iter()
        .map(|x| x.number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .sum();
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    // 560513

    #[test]
    fn test_part_1() {

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
        let expected_result = 4361;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_1_only_symbol_between_numbers() {
        let test_input = "50*50..";
        let expected_result = 100;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_1_single_numbers() {
        let test_input = "
            .5*...
            ...8..";
        let expected_result = 13;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_1_many_adj() {
        let test_input = "
            .*50*...
            ..=/....";
        let expected_result = 50;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_1_end() {
        let test_input = "
            .....*50
            ..=/....";
        let expected_result = 50;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}


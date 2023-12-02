fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn convert_to_digit(input: &str) -> Option<char> {
    match input {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut digits: Vec<char> = vec![];
        let mut possible_digits: Vec<String> = vec![];
        for c in line.chars() {
            if c.is_digit(10) {
                digits.push(c);
                continue;
            }

            possible_digits.push("".to_string());

            let n = possible_digits.len();
            for i in 0..n {
                possible_digits[i].push(c);
            }

            for i in 0..n {
                let converted_digit = convert_to_digit(&possible_digits[i]);

                if !converted_digit.is_none() {
                    digits.push(converted_digit.unwrap());
                    possible_digits.remove(i);
                    break;
                }
            }
        }

        let v = vec![digits[0], digits[digits.len() - 1]];

        let n_str: String = v.into_iter().collect();
        let n: u32 = n_str.parse::<u32>().unwrap();
        sum += n;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let test_input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected_result = 281;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_2_overlap() {
        let test_input = "eighttwone";
        let expected_result = 81;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}

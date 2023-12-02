fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut digits = vec![];
        for c in line.chars() {
            if !c.is_digit(10) {
                continue;
            }

            digits.push(c);
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
    fn test_part_1() {
        let test_input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected_result: u32 = 142;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}

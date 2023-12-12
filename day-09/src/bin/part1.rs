fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_differences(numbers: &Vec<i32>) -> Vec<i32> {
    let mut diffs = vec![];
    let mut i = 0;
    while i + 1 < numbers.len() {
        diffs.push(numbers[i + 1] - numbers[i]);
        i += 1
    }
    return diffs;
}

fn history(numbers: &Vec<i32>) -> i32 {
    let all_same = numbers.iter().all(|&x| x == 0);
    if all_same {
        return 0;
    }
    let differences: Vec<i32> = get_differences(numbers);
    let h = history(&differences);
    return differences[differences.len() - 1] + h;
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let h = history(&numbers);
        sum += numbers[numbers.len() - 1] + h;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {

        let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected_result = 114;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}


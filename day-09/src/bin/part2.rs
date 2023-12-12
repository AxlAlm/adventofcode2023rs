fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
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
    return differences[0] - h;
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let h = history(&numbers);
        sum += numbers[0] - h;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {

        let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected_result = 2;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }

    // #[test]
    // fn test_part2_minues() {
    //     let test_input = "14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 -1 -2 -3 -4 -5 -6";
    //     let expected_result = -7;
    //     let result = part2(test_input);
    //     assert_eq!(expected_result, result);
    // }

    // #[test]
    // fn test_part2_shifting() {
    //     let test_input = "0 5 10 -15 30 -30";
    //     let expected_result = -7;
    //     let result = part2(test_input);
    //     assert_eq!(expected_result, result);
    // }
}


fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    return "not implemented".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let test_input = "";
        let expected_result = "not implemented";
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}

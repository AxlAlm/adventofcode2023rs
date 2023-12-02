fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    return "not implemented".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = "";
        let expected_result = "not implemented";
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}

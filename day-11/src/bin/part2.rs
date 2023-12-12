fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let test_input = "";
        let expected_result = 0;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}

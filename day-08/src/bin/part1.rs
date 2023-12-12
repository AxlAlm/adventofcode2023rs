use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_network(input: &str) -> (HashMap<String, Vec<String>>, Vec<&str>) {
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    let mut instructions: Vec<&str> = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            instructions = line.trim().split("").filter(|x| x != &"").collect();
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let line = line.replace(")", "");
        let line = line.replace("(", "");
        let line = line.replace("= ", "");
        let line = line.replace(",", "");
        let nodes: Vec<&str> = line.split(" ").collect();

        network.insert(
            nodes[0].to_string(),
            vec![nodes[1].to_string(), nodes[2].to_string()],
        );
    }

    return (network, instructions);
}

fn instruction_to_index(instruction: &str) -> usize {
    match instruction {
        "L" => 0,
        "R" => 1,
        _ => panic!("Invalid Instruction"),
    }
}

fn part1(input: &str) -> u32 {
    let (network, instructions) = parse_network(input);

    dbg!(&instructions);

    let mut nr_steps = 0;
    let mut current_node = "AAA".to_string();
    loop {
        for instr in instructions.iter() {
            nr_steps += 1;

            let i = instruction_to_index(instr);
            let available_nodes = network.get(&current_node).unwrap();

            // dbg!(current_node, &available_nodes[i], nr_steps);
            current_node = available_nodes[i].to_owned();

            if current_node == "ZZZ" {
                return nr_steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_network1() {

        let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let expected_result = 2;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part1_network2() {

        let test_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let expected_result = 6;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}


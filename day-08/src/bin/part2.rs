use std::collections::HashMap;

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
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

        let line = line
            .replace(")", "")
            .replace("(", "")
            .replace("= ", "")
            .replace(",", "");
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

fn steps_to_z(
    instructions: &Vec<&str>,
    network: &HashMap<String, Vec<String>>,
    start_node: String,
) -> u32 {
    let mut nr_steps = 0;
    let mut current_node = start_node;
    loop {
        for instr in instructions.iter() {
            nr_steps += 1;

            let i = instruction_to_index(instr);
            let available_nodes = network.get(&current_node).unwrap();

            // dbg!(current_node, &available_nodes[i], nr_steps);
            current_node = available_nodes[i].to_owned();

            if current_node.ends_with("Z") {
                return nr_steps;
            }
        }
    }
}

fn part2(input: &str) -> u64 {
    let (network, instructions) = parse_network(input);

    let starting_nodes: Vec<&String> = network.keys().filter(|k| k.ends_with("A")).collect();

    // let mut current_nodes: HashMap<String, String> = HashMap::new();
    // for start_node in starting_nodes.iter() {
    //     current_nodes.insert(start_node.to_string(), start_node.to_string());
    // }

    let mut nr_total: u64 = 1;
    for start_node in starting_nodes.iter() {
        let n_steps = steps_to_z(&instructions, &network, start_node.to_string());
        dbg!(n_steps);
        // nr_total *= n_steps as u64;
    }

    return nr_total;

    // let mut nr_steps = 0;
    // loop {
    //     for instr in instructions.iter() {
    //         let i = instruction_to_index(instr);
    //         nr_steps += 1;
    //         let mut nr_end_with_z = 0;
    //         for start_node in starting_nodes.iter() {
    //             // println!("--------------------");
    //             // dbg!(&current_nodes);

    //             let current_node = current_nodes.get(start_node.to_owned()).unwrap().clone();
    //             let available_nodes = network.get(&current_node.to_owned()).unwrap();
    //             let chosen_node = available_nodes[i].to_owned();
    //             current_nodes.insert(start_node.to_string().clone(), chosen_node.clone());

    //             if chosen_node.ends_with("Z") {
    //                 nr_end_with_z += 1
    //             }

    //             // dbg!(
    //             //     &start_node,
    //             //     &current_node,
    //             //     &available_nodes,
    //             //     i,
    //             //     &chosen_node,
    //             //     nr_steps,
    //             //     &current_nodes,
    //             // );
    //         }

    //         if nr_steps % 1000000 == 0 {
    //             dbg!(nr_steps);
    //         }
    //         if nr_end_with_z == starting_nodes.len() {
    //             return nr_steps;
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_network1() {

        let test_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX";
        let expected_result = 6;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}


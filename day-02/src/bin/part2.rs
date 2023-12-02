#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Clone)]
struct Set {
    nr_red_cubes: u32,
    nr_green_cubes: u32,
    nr_blue_cubes: u32,
}

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn create_game(input: &str) -> Game {
    let game_id_sets: Vec<&str> = input.trim().split(":").collect();
    let game_id: Vec<&str> = game_id_sets[0].split_whitespace().collect();
    let id: u32 = game_id[1].parse::<u32>().unwrap();
    let mut sets: Vec<Set> = vec![];
    for set_str in game_id_sets[1].split(";") {
        let mut set = Set {
            nr_red_cubes: 0,
            nr_green_cubes: 0,
            nr_blue_cubes: 0,
        };
        for cube_info_str in set_str.split(",") {
            let n_color: Vec<&str> = cube_info_str.trim().split_whitespace().collect();
            let color = n_color[1];
            let n = n_color[0].parse::<u32>().unwrap();

            match color {
                "red" => set.nr_red_cubes = n,
                "green" => set.nr_green_cubes = n,
                "blue" => set.nr_blue_cubes = n,
                _ => panic!("Failed to find matching color"),
            }
        }

        sets.push(set)
    }

    return Game { id: id, sets: sets };
}

fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let game = create_game(line);

        let mut minimum_set = Set {
            nr_red_cubes: 0,
            nr_green_cubes: 0,
            nr_blue_cubes: 0,
        };
        for set in game.sets.iter() {
            minimum_set.nr_red_cubes = std::cmp::max(set.nr_red_cubes, minimum_set.nr_red_cubes);
            minimum_set.nr_green_cubes =
                std::cmp::max(set.nr_green_cubes, minimum_set.nr_green_cubes);
            minimum_set.nr_blue_cubes = std::cmp::max(set.nr_blue_cubes, minimum_set.nr_blue_cubes);
        }

        sum += minimum_set.nr_red_cubes * minimum_set.nr_green_cubes * minimum_set.nr_blue_cubes
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {

        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_result = 2286;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}


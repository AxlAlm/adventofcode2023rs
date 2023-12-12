use core::panic;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let mut current_pipe = "F".to_string();
    let mut from_cardinal = "w".to_string();
    let mut to_cardinal = "e".to_string();

    let (start_x, start_y) = get_start_position(&grid);
    let (mut x, mut y) = (start_x.clone(), start_y.clone());

    let mut nr_steps = 0;
    loop {
        nr_steps += 1;
        let (x_offset, y_offset) = get_offset_from_cardinal(&to_cardinal);
        x += x_offset;
        y += y_offset;
        current_pipe = grid[y as usize][x as usize].to_string();

        if current_pipe == "S" {
            break;
        }
        to_cardinal = get_cardinal_from_pipe(&from_cardinal, &current_pipe);
        from_cardinal = cardinal_rev(&to_cardinal);
    }

    return nr_steps / 2;
}

fn get_start_position(grid: &Vec<Vec<&str>>) -> (i32, i32) {
    for (y, v) in grid.iter().enumerate() {
        for (x, pipe) in v.into_iter().enumerate() {
            if pipe == &"S" {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start found");
}

fn parse_grid(input: &str) -> Vec<Vec<&str>> {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|x| {
            x.trim()
                .split("")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>()
        })
        .collect();

    return grid;
}

fn cardinal_rev(from_cardinal: &str) -> String {
    match from_cardinal {
        "s" => "n".to_string(),
        "n" => "s".to_string(),
        "w" => "e".to_string(),
        "e" => "w".to_string(),
        _ => panic!("No pipe matching"),
    }
}

fn get_offset_from_cardinal(to_cardinal: &str) -> (i32, i32) {
    match to_cardinal {
        "n" => (0, -1),
        "s" => (0, 1),
        "e" => (1, 0),
        "w" => (-1, 0),
        _ => panic!("No pipe matching"),
    }
}

fn get_cardinal_from_pipe(from_cardinal: &str, pipe: &str) -> String {
    match (from_cardinal, pipe) {
        ("n", "|") => "s".to_string(),
        ("s", "|") => "n".to_string(),
        ("e", "-") => "w".to_string(),
        ("w", "-") => "e".to_string(),

        ("n", "L") => "e".to_string(),
        ("e", "L") => "n".to_string(),
        ("n", "J") => "w".to_string(),
        ("w", "J") => "n".to_string(),

        ("w", "7") => "s".to_string(),
        ("s", "7") => "w".to_string(),

        ("s", "F") => "e".to_string(),
        ("e", "F") => "s".to_string(),
        _ => panic!("No pipe matching"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {

        let test_input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let expected_result = 4;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part1_example2() {
        let test_input = "7-F7-
    .FJ|7
    SJLL7
    |F--J
    LJ.LJ";
        let expected_result = 8;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}


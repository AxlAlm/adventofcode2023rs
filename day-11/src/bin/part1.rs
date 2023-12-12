use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let galaxies = parse_graph(input);
    let mut sum = 0;
    let mut results: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, g) in galaxies.iter().enumerate() {
        for (j, gg) in galaxies.iter().enumerate() {
            let is_calc = results.contains_key(&(i, j)) || results.contains_key(&(j, i));
            if i == j || is_calc {
                continue;
            }

            sum += manhattan_distance(g, gg);
            results.insert((i, j), manhattan_distance(g, gg));
        }
    }
    return sum as u32;
}

fn parse_graph(input: &str) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    let mut rows = vec![];

    let mut y2expy: HashMap<usize, usize> = HashMap::new();
    let mut expy = 0;
    for (y, line) in input.lines().enumerate() {
        y2expy.insert(y, expy);
        let no_galaxy = !line.contains("#");
        expy += if no_galaxy { 2 } else { 1 };
        let row: Vec<&str> = line.trim().split("").filter(|x| x != &"").collect();
        rows.push(row);
    }

    let mut x2expx: HashMap<usize, usize> = HashMap::new();
    let mut expx = 0;
    for x in 0..rows[0].len() {
        x2expx.insert(x, expx);
        let mut col = vec![];
        for row in rows.iter() {
            col.push(row[x])
        }
        let no_galaxy = col.iter().all(|&element| element == ".");
        expx += if no_galaxy { 2 } else { 1 };
    }

    for (y, row) in rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell != &"#" {
                continue;
            }
            galaxies.push((
                x2expx.get(&x).unwrap().clone(),
                y2expy.get(&y).unwrap().clone(),
            ));
        }
    }

    return galaxies;
}

fn manhattan_distance(from_xy: &(usize, usize), to_xy: &(usize, usize)) -> usize {
    return abs_diff(from_xy.0, to_xy.0) + abs_diff(from_xy.1, to_xy.1);
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y {
        return x - y;
    } else {
        return y - x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {

        let test_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected_result = 374;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}


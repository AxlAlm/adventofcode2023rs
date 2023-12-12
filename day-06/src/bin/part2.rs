use itertools::Itertools;

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Race {
    time_ms: u64,
    record_distance_mm: u64,
}

fn find_nr_ways_to_win(race: &Race) -> u64 {
    let mut time_ms = race.time_ms;
    let mut speed = 0;
    let mut nr_ways_to_win = 0;
    for _ in 0..time_ms {
        let traveled_mm = speed * time_ms;
        if traveled_mm > race.record_distance_mm {
            nr_ways_to_win += 1
        }

        speed += 1;
        time_ms -= 1;
    }

    return nr_ways_to_win;
}

fn parse_race(input: &str) -> Race {
    let lines: Vec<&str> = input.split("\n").collect();
    let times: Vec<&str> = lines[0]
        .strip_prefix("Time: ")
        .unwrap_or("")
        .split_whitespace()
        .collect();

    let distances: Vec<&str> = lines[1]
        .strip_prefix("Distance: ")
        .unwrap_or("")
        .split_whitespace()
        .collect();

    return Race {
        time_ms: times.join("").parse::<u64>().unwrap(),
        record_distance_mm: distances.join("").parse::<u64>().unwrap(),
    };
}

fn part2(input: &str) -> u64 {
    let race = parse_race(input);
    return find_nr_ways_to_win(&race);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {

        let test_input = "Time:      7  15   30
Distance:  9  40  200";
        let expected_result = 71503;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}


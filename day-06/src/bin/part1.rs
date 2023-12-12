fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Race {
    time_ms: u32,
    record_distance_mm: u32,
}

fn find_nr_ways_to_win(race: &Race) -> u32 {
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

fn parse_races(input: &str) -> Vec<Race> {
    let mut races = vec![];

    let lines: Vec<&str> = input.split("\n").collect();
    let times: Vec<u32> = lines[0]
        .strip_prefix("Time: ")
        .unwrap_or("")
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = lines[1]
        .strip_prefix("Distance: ")
        .unwrap_or("")
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // dbg!(&distances);
    // dbg!(&times);

    for i in 0..times.len() {
        races.push(Race {
            time_ms: times[i],
            record_distance_mm: distances[i],
        });
    }

    return races;
}

fn part1(input: &str) -> u32 {
    let races = parse_races(input);

    let mut total_ways_to_win = 1;
    for race in races.iter() {
        // dbg!(race);
        // dbg!(find_nr_ways_to_win(race));
        total_ways_to_win *= find_nr_ways_to_win(race);
    }
    return total_ways_to_win;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {

        let test_input = "Time:      7  15   30
Distance:  9  40  200";
        let expected_result = 288;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part_1_races() {
        let race = Race {
            time_ms: 7,
            record_distance_mm: 9,
        };
        let expected_result = 4;
        let result = find_nr_ways_to_win(&race);
        assert_eq!(expected_result, result);
    }
}


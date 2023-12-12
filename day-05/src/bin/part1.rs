use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Map {
    dst_start_nr: u64,
    src_start_nr: u64,
    range_length: u64,
}

#[derive(Debug, Clone)]
struct CategoryMaps {
    src_cat: String,
    dst_cat: String,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Almnac {
    cat_maps: HashMap<String, CategoryMaps>,
    seeds: Vec<u64>,
}

fn parse_almnac(input: &str) -> Almnac {
    let input = input.replace("\r", "");
    let mut cat_maps: HashMap<String, CategoryMaps> = HashMap::new();
    let blocks: Vec<&str> = input.split("\n\n").into_iter().collect();
    let seeds: Vec<u64> = blocks[0]
        .strip_prefix("seeds: ")
        .unwrap_or("")
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for block in blocks[1..].iter() {
        if block.trim().is_empty() {
            continue;
        }

        let lines: Vec<&str> = block.trim().split("\n").into_iter().collect();
        let map_name = lines[0].strip_suffix(" map:").unwrap();
        let map_cats: Vec<&str> = map_name.split("-to-").into_iter().collect();
        let src_name = map_cats[0].to_string();
        let dst_name = map_cats[1].to_string();

        for line in lines[1..].iter() {
            if line.trim().is_empty() {
                continue;
            }

            let map_numbers: Vec<u64> = line
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            if !cat_maps.contains_key(&src_name) {
                cat_maps.insert(
                    src_name.to_owned(),
                    CategoryMaps {
                        src_cat: src_name.to_owned(),
                        dst_cat: dst_name.to_owned(),
                        maps: vec![],
                    },
                );
            }

            let m = Map {
                dst_start_nr: map_numbers[0],
                src_start_nr: map_numbers[1],
                range_length: map_numbers[2],
            };

            cat_maps.get_mut(&src_name).unwrap().maps.push(m);
        }
    }

    return Almnac {
        cat_maps: cat_maps.to_owned(),
        seeds: seeds.to_owned(),
    };
}

fn find_dst(maps: &Vec<Map>, src: u64) -> u64 {
    let matched_dst = src;
    for map in maps.iter() {
        if src <= map.src_start_nr {
            continue;
        }
        let x = src - map.src_start_nr;
        if x <= map.range_length {
            return map.dst_start_nr + x;
        }
    }
    return matched_dst;
}

fn map_src_to_dst(almnac: &Almnac, src: u64, map_key: &str) -> u64 {
    let cat_map = &almnac.cat_maps[map_key];
    let mut dst = find_dst(&cat_map.maps, src);

    if cat_map.dst_cat != "location" {
        dst = map_src_to_dst(almnac, dst, &cat_map.dst_cat);
    }

    return dst;
}

fn find_lowest_location(almnac: &Almnac) -> u64 {
    let mut lowest_location = 99999999999999;
    let start_key = "seed";

    for seed in almnac.seeds.iter() {
        let location = map_src_to_dst(almnac, seed.to_owned(), start_key);
        if location < lowest_location {
            lowest_location = location
        }
    }

    return lowest_location;
}

fn part1(input: &str) -> u64 {
    let almnac = parse_almnac(input);
    return find_lowest_location(&almnac);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {

        let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
            ";
        let expected_result = 35;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}


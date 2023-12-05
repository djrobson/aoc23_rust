aoc23_rust::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_lines = input.lines();
    let seeds: Vec<usize> = input_lines
        .next()
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|seeds| seeds.split(' ').map(|s| s.parse().unwrap()).collect())
        .unwrap();
    let mut map_index = 0;
    input_lines.next(); // skip the first blank line

    let mut maps: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    maps.push(Vec::new());
    for line in input_lines {
        if line.eq("") {
            map_index += 1;
            maps.push(Vec::new());
        } else if line.ends_with(':') {
            // this is the name of the new map
        } else {
            let nums: Vec<usize> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            maps[map_index].push((nums[0], nums[1], nums[2]));
        }
    }

    let mut min_seed = usize::MAX;
    for seed in seeds {
        let mut mapped_seed = seed;
        for map in &maps {
            //let mut found_rule = false;
            'rules: for rule in map {
                if mapped_seed >= rule.1 && mapped_seed < (rule.1 + rule.2) {
                    mapped_seed = (mapped_seed - rule.1) + rule.0;
                    //found_rule = true;
                    break 'rules;
                }
            }
        }
        min_seed = min_seed.min(mapped_seed);
    }

    Some(min_seed as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

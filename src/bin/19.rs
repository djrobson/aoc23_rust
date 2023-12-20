aoc23_rust::solution!(19);
use std::collections::HashMap;

struct Rule {
    member: u8,
    oper: char,
    value: u32,
    dest: String,
}

fn get_member_from_letter(letter: char) -> u8 {
    match letter {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Invalid letter"),
    }
}

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<(i32, i32, i32, i32)>) {
    // split input into a first and second section with a blank line in the middle
    let mut sections = input.split("\n\n");

    // parse each line of section 1 into a vector of Strings
    let rule_strings = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    //parse each rule_string
    // trim off the } at the end
    // split all chars before { into the name
    // after the { split on comma
    // parse each segment into a rule
    // store the rule into a hashmap based on the name

    let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
    rule_strings.iter().for_each(|rule| {
        let name_vals = rule.trim_end_matches('}').split('{').collect::<Vec<&str>>();
        let name = name_vals[0].to_string();

        // split the seconrd half into a vec of rules
        let segments: Vec<&str> = name_vals[1].split(',').collect();
        rules.insert(name.clone(), Vec::new());

        segments.iter().for_each(|segment| {
            // if segment is like a<2006:qkq parse into a rule
            let rule = if segment.contains(':') {
                let mut seg_vec = segment.chars();
                let member = get_member_from_letter(seg_vec.next().unwrap());
                let oper = seg_vec.next().unwrap();
                let value_dest = segment[2..].split(':').collect::<Vec<&str>>();
                let value = value_dest[0].parse::<u32>().unwrap();
                let dest = value_dest[1].to_string();

                Rule {
                    member,
                    oper,
                    value,
                    dest,
                }
            } else {
                // this rule should always succeed
                Rule {
                    member: 0,
                    oper: '>',
                    value: 0,
                    dest: name.clone(),
                }
            };
            rules.get_mut(&name).unwrap().push(rule);
        });

        // if segment is like pv then parse into a rule
    });

    // parse each line of section 2
    // trim the { and } off the ends
    // split the interior on ,
    // the names always exist in the same order, so throw them out
    // store the resulting name and value into a tuple of i32s
    // store the tuple into a vector
    let values: Vec<(i32, i32, i32, i32)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let vals = line
                .trim_end_matches('}')
                .trim_start_matches('{')
                .split(',')
                .map(|name_val| {
                    // throw away the front 2 chars and parse the remained to an i32
                    name_val[2..].parse::<i32>().unwrap()
                })
                .collect::<Vec<i32>>();
            (vals[0], vals[1], vals[2], vals[3])
        })
        .collect();

    (rules, values)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, values) = parse_input(input);
    let mut accepted: u32 = 0;
    let mut rejected: u32 = 0;

    

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

aoc23_rust::solution!(19);
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Rule {
    member: u8,
    oper: char,
    value: u32,
    dest: String,
}

fn eval_rule(value: &(u32, u32, u32, u32), rule: &Rule) -> Option<String> {
    let conditional_jump = match rule.oper {
        '>' => match rule.member {
            0 => value.0 > rule.value,
            1 => value.1 > rule.value,
            2 => value.2 > rule.value,
            3 => value.3 > rule.value,
            _ => panic!("Invalid member"),
        },
        '<' => match rule.member {
            0 => value.0 < rule.value,
            1 => value.1 < rule.value,
            2 => value.2 < rule.value,
            3 => value.3 < rule.value,
            _ => panic!("Invalid member"),
        },
        _ => panic!("Invalid operator"),
    };

    if conditional_jump {
        Some(rule.dest.clone())
    } else {
        None
    }
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

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<(u32, u32, u32, u32)>) {
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
                let dest = segment.to_string();
                // this rule should always succeed
                Rule {
                    member: 0,
                    oper: '>',
                    value: 0,
                    dest: dest,
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
    let values: Vec<(u32, u32, u32, u32)> = sections
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
                    name_val[2..].parse::<u32>().unwrap()
                })
                .collect::<Vec<u32>>();
            (vals[0], vals[1], vals[2], vals[3])
        })
        .collect();

    (rules, values)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, values) = parse_input(input);

    let mut accepted: u32 = 0;
    //let mut rejected: u32 = 0;

    // for each value we recieved, evaluate the rules until accepted or rejected
    for value in values {
        //  start at IN and work until you find
        let mut rules_seen: HashSet<String> = HashSet::new();
        let mut rule_vec_name = "in".to_string();
        'process_value: loop {
            if rules_seen.contains(&rule_vec_name) {
                panic!("Infinite loop detected {}", rule_vec_name);
            }
            rules_seen.insert(rule_vec_name.clone());

            let rule_vec = rules.get(&rule_vec_name).unwrap();
            let mut next_rule;
            'process_rules: for rule in rule_vec {
                next_rule = eval_rule(&value, rule);

                if next_rule.is_some() {
                    if next_rule == Some("A".to_string()) {
                        accepted += value.0 + value.1 + value.2 + value.3;
                        break 'process_value;
                    } else if next_rule == Some("R".to_string()) {
                        break 'process_value;
                    } else {
                        rule_vec_name = next_rule.unwrap();
                        break 'process_rules;
                    }
                }
            }
        }
    }

    Some(accepted)
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

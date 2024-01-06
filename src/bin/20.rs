aoc23_rust::solution!(20);

use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum WireState {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Event {
    from: String,
    time: u32,
    module: String,
    state: WireState,
}

#[derive(PartialEq, Copy, Clone)]
enum ModuleType {
    FlipFlip,
    Conjunction,
    Broadcaster,
    Output,
}

#[derive(Clone)]
struct Module {
    inputs: HashMap<String, WireState>,
    outputs: Vec<String>,
    state: WireState,
    name: String,
    mod_type: ModuleType,
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let name = parts.next().unwrap();
        let mod_type: ModuleType;
        let mod_name: String;
        (mod_type, mod_name) = match name.chars().next().unwrap() {
            '%' => (ModuleType::FlipFlip, name[1..].to_string()),
            '&' => (ModuleType::Conjunction, name[1..].to_string()),
            'b' => (ModuleType::Broadcaster, name.to_string()),
            'o' => (ModuleType::Output, name.to_string()),
            _ => panic!("Unknown module type"),
        };

        let mut module = Module {
            outputs: Vec::new(),
            inputs: HashMap::new(),
            state: WireState::Low,
            name: mod_name,
            mod_type,
        };
        let outputs = parts.next().unwrap();
        for output in outputs.split(", ") {
            module.outputs.push(output.to_string());
        }
        modules.insert(module.name.clone(), module);
    }

    let modules_copy = modules.clone();
    // for each module
    for module_name in modules_copy.keys() {
        // check if anyone else outputs to us
        for module in &modules_copy {
            // keep a record of who outputs to us in our inputs
            if module.1.outputs.contains(module_name) {
                modules
                    .get_mut(module_name)
                    .unwrap()
                    .inputs
                    .insert(module.0.clone(), WireState::Low);
            }
        }
    }
    modules
}

fn process_event(event: &Event, modules: &mut HashMap<String, Module>) -> VecDeque<Event> {
    let mut new_events: VecDeque<Event> = VecDeque::new();
    let module = modules
        .get_mut(&event.module)
        .unwrap_or_else(|| panic!("Unknown module {}", event.module));
    match module.mod_type {
        ModuleType::FlipFlip => {
            if event.state == WireState::Low {
                module.state = match module.state {
                    WireState::Low => WireState::High,
                    WireState::High => WireState::Low,
                };

                for output in &module.outputs {
                    new_events.push_back(Event {
                        from: module.name.clone(),
                        time: event.time + 1,
                        module: output.clone(),
                        state: module.state,
                    });
                }
            }
        }
        ModuleType::Conjunction => {
            let mut all_high = true;

            module.inputs.insert(event.from.clone(), event.state);
            for input in &module.inputs {
                if input.1 == &WireState::Low {
                    all_high = false;
                    break;
                }
            }

            if all_high {
                for output in &module.outputs {
                    new_events.push_back(Event {
                        from: module.name.clone(),
                        time: event.time + 1,
                        module: output.clone(),
                        state: WireState::Low,
                    });
                }
            } else {
                for output in &module.outputs {
                    new_events.push_back(Event {
                        from: module.name.clone(),
                        time: event.time + 1,
                        module: output.clone(),
                        state: WireState::High,
                    });
                }
            }
        }

        ModuleType::Broadcaster => {
            for output in &module.outputs {
                new_events.push_back(Event {
                    from: module.name.clone(),
                    time: event.time + 1,
                    module: output.clone(),
                    state: event.state,
                });
            }
        }

        ModuleType::Output => {
            //println!("output event {:?}", event);
        }
    }
    new_events
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules: HashMap<String, Module> = parse_input(input);
    let mut low_pulse = 0;
    let mut high_pulse = 0;
    let mut event_queue: VecDeque<Event> = VecDeque::new();

    for tick in 0..1000 {
        // tap the button once per iteration
        event_queue.push_back(Event {
            from: "button".to_string(),
            time: tick,
            module: "broadcaster".to_string(),
            state: WireState::Low,
        });

        while let Some(event) = event_queue.pop_front() {
            //println!("{} {:?} {}", event.from, event.state, event.module);
            match event.state {
                WireState::Low => {
                    low_pulse += 1;
                }
                WireState::High => {
                    high_pulse += 1;
                }
            }

            event_queue.append(&mut process_event(&event, &mut modules));
        }
    }
    Some(high_pulse * low_pulse)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(result, Some(32_000_000));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
output -> ",
        );
        assert_eq!(result, Some(11687500));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

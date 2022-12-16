use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::rc::Rc;
use std::vec::Vec;

struct Valve {
    flow: u32,
    tunnels: Vec<String>,
}

struct SearchState {
    time: u8,
    valve: String,
    pressure: u32,
    flow_rate: u32,
    opened: HashSet<String>,
}

struct TargetSearchState {
    valve: String,
    visited: Rc<HashSet<String>>,
}

fn main() {
    let mut valves = HashMap::new();
    let mut nonzero_valves = HashSet::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (valve_str, tunnel_str) = line
            .split_once(';')
            .expect("line should have valve and tunnels");

        let name = String::from(&valve_str[6..8]);
        let flow: u32 = valve_str[23..]
            .parse()
            .expect("flow rate should be numeric");
        let tunnels: Vec<String> = tunnel_str[{
            if tunnel_str.chars().nth(22).unwrap() == 's' {
                24..
            } else {
                23..
            }
        }]
        .split(", ")
        .map(String::from)
        .collect();

        valves.insert(
            name.clone(),
            Valve {
                flow: flow,
                tunnels: tunnels,
            },
        );
        if flow > 0 {
            nonzero_valves.insert(name);
        }
    }

    let mut max = 0;
    let mut search = vec![SearchState {
        time: 1,
        valve: String::from("AA"),
        pressure: 0,
        flow_rate: 0,
        opened: HashSet::new(),
    }];

    while !search.is_empty() {
        let state = &search.pop().unwrap();

        // Try going to all unopened, non-zero valves.
        let mut target_valves: HashSet<String> = nonzero_valves
            .difference(&state.opened)
            .map(|s| s.to_string())
            .collect();

        let mut target_search = VecDeque::from([TargetSearchState {
            valve: state.valve.clone(),
            visited: Rc::new(HashSet::new()),
        }]);
        'target_search_loop: while !target_valves.is_empty() {
            let target_state = target_search
                .pop_front()
                .expect("target search stack should always have something");
            let v = &target_state.valve;
            if state.time as usize + target_state.visited.len() > 29 {
                break 'target_search_loop;
            }

            let cur_valve = valves.get(v).unwrap();

            if target_valves.contains(v) {
                let time_taken = target_state.visited.len() + 1;
                let mut new_opened = state.opened.clone();
                new_opened.insert(v.clone());

                search.push(SearchState {
                    time: state.time + time_taken as u8,
                    valve: v.clone(),
                    pressure: state.pressure + state.flow_rate * time_taken as u32,
                    flow_rate: state.flow_rate + cur_valve.flow,
                    opened: new_opened,
                });

                target_valves.remove(&target_state.valve);
            }

            let mut new_visited = (*target_state.visited).clone();
            new_visited.insert(v.clone());
            let new_visited_rc = Rc::new(new_visited);

            for tunnel in &cur_valve.tunnels {
                if !target_state.visited.contains(tunnel) {
                    target_search.push_back(TargetSearchState {
                        valve: tunnel.clone(),
                        visited: Rc::clone(&new_visited_rc),
                    });
                }
            }
        }

        // Account for idle state from this position.
        let idle_min = 31 - state.time;
        let pressure = state.pressure + state.flow_rate * idle_min as u32;
        if pressure > max {
            max = pressure;
        }
    }

    println!("{}", max);
}

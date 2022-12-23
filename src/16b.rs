use std::cmp;
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
    searcher: [SearcherState; 2],
    pressure: u32,
    flow_rate: u32,
    to_open: HashSet<String>,
}

struct SearcherState {
    valve: String,
    // 0 means searcher is at valve, else searcher will open valve in "distance"
    // minutes.
    distance: u8,
}

struct TargetSearchState {
    valve: String,
    visited: Rc<HashSet<String>>,
}

fn fill_dist_map(
    dist_memo: &mut HashMap<(String, String), u8>,
    valves: &HashMap<String, Valve>,
    from: &String,
    to_open: &HashSet<String>,
    max: u8,
) {
    // Ignore destinations that already have distance information filled.
    let mut search_for = to_open.clone();
    search_for
        .retain(|target_valve| !dist_memo.contains_key(&(from.clone(), target_valve.clone())));

    let mut target_search = VecDeque::from([TargetSearchState {
        valve: from.clone(),
        visited: Rc::new(HashSet::new()),
    }]);

    while !search_for.is_empty() {
        let target_state = target_search
            .pop_front()
            .expect("target search stack should always have something");
        let v = &target_state.valve;
        if target_state.visited.len() as u8 > max {
            return;
        }

        let cur_valve = valves.get(v).unwrap();

        if search_for.contains(v) {
            let time_taken = (target_state.visited.len() + 1) as u8;
            dist_memo.insert((from.clone(), v.clone()), time_taken);
            dist_memo.insert((v.clone(), from.clone()), time_taken);

            search_for.remove(&target_state.valve);
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
        searcher: [
            SearcherState {
                valve: String::from("AA"),
                distance: 0,
            },
            SearcherState {
                valve: String::from("AA"),
                distance: 0,
            },
        ],
        pressure: 0,
        flow_rate: 0,
        to_open: nonzero_valves,
    }];
    // dist_memo will contain time taken to go from one tunnel to another.
    let mut dist_memo: HashMap<(String, String), u8> = HashMap::new();

    while !search.is_empty() {
        let state = &search.pop().unwrap();

        if state.searcher[0].distance == 0 && state.searcher[1].distance == 0 {
            // Both searchers need new targets.
            let time_left = 25 - state.time as u8;
            let from_0 = &state.searcher[0].valve;
            let from_1 = &state.searcher[1].valve;

            fill_dist_map(&mut dist_memo, &valves, &from_0, &state.to_open, time_left);
            fill_dist_map(&mut dist_memo, &valves, &from_1, &state.to_open, time_left);

            if from_0 == from_1 {
                // Since both searchers are currently at the same valve,
                // symmetric assignments of 2 tunnels are equivalent. Send
                // searcher 0 to the closer tunnel of the 2 to reduce search
                // state space since both combinations would yield the same
                // result.
                let mut target_list: Vec<&String> = state
                    .to_open
                    .iter()
                    .filter(|v| {
                        dist_memo
                            .get(&(from_1.clone(), v.to_string()))
                            .map_or(false, |&d| d < time_left)
                    })
                    .collect();
                target_list.sort_unstable_by_key(|v| {
                    dist_memo.get(&(from_1.clone(), v.to_string())).unwrap()
                });

                if target_list.len() == 1 {
                    // Only one target left, send searcher to last target.
                    let target = target_list[0];
                    let mut new_to_open = state.to_open.clone();
                    new_to_open.remove(target);

                    search.push(SearchState {
                        time: state.time,
                        searcher: [
                            SearcherState {
                                valve: target.clone(),
                                distance: *dist_memo
                                    .get(&(from_1.clone(), target.clone()))
                                    .unwrap(),
                            },
                            SearcherState {
                                valve: from_1.clone(),
                                distance: 0,
                            },
                        ],
                        pressure: state.pressure,
                        flow_rate: state.flow_rate,
                        to_open: new_to_open,
                    });
                } else {
                    for j in 1..target_list.len() {
                        let valve_j = target_list[j];
                        for i in 0..j {
                            let mut new_to_open = state.to_open.clone();
                            let valve_i = target_list[i];
                            new_to_open.remove(valve_i);
                            new_to_open.remove(valve_j);

                            search.push(SearchState {
                                time: state.time,
                                searcher: [
                                    SearcherState {
                                        valve: valve_i.clone(),
                                        distance: *dist_memo
                                            .get(&(from_0.clone(), valve_i.clone()))
                                            .unwrap(),
                                    },
                                    SearcherState {
                                        valve: valve_j.clone(),
                                        distance: *dist_memo
                                            .get(&(from_0.clone(), valve_j.clone()))
                                            .unwrap(),
                                    },
                                ],
                                pressure: state.pressure,
                                flow_rate: state.flow_rate,
                                to_open: new_to_open,
                            });
                        }
                    }
                }
            } else {
                let target_list_0: Vec<&String> = state
                    .to_open
                    .iter()
                    .filter(|v| {
                        dist_memo
                            .get(&(from_0.clone(), v.to_string()))
                            .map_or(false, |&d| d < time_left)
                    })
                    .collect();
                let target_list_1: Vec<&String> = state
                    .to_open
                    .iter()
                    .filter(|v| {
                        dist_memo
                            .get(&(from_1.clone(), v.to_string()))
                            .map_or(false, |&d| d < time_left)
                    })
                    .collect();

                if target_list_0.is_empty() && target_list_1.is_empty() {
                    // Neither searcher has candidate targets, so just end.
                } else if !target_list_0.is_empty() && !target_list_1.is_empty() {
                    // Both searchers have candidate targets, so add every
                    // combination of targets to search.
                    for t0 in &target_list_0 {
                        for t1 in &target_list_1 {
                            if t0 == t1 {
                                // Don't move both searchers to the same target,
                                // as that would cause a double count. Try
                                // either searcher going to the target (the next
                                // iteration will pick a target for the other
                                // searcher).
                                let mut new_to_open = state.to_open.clone();
                                new_to_open.remove(*t0);

                                search.push(SearchState {
                                    time: state.time,
                                    searcher: [
                                        SearcherState {
                                            valve: t0.to_string(),
                                            distance: *dist_memo
                                                .get(&(from_0.clone(), t0.to_string()))
                                                .unwrap(),
                                        },
                                        SearcherState {
                                            valve: state.searcher[1].valve.clone(),
                                            distance: 0,
                                        },
                                    ],
                                    pressure: state.pressure,
                                    flow_rate: state.flow_rate,
                                    to_open: new_to_open.clone(),
                                });
                                search.push(SearchState {
                                    time: state.time,
                                    searcher: [
                                        SearcherState {
                                            valve: state.searcher[0].valve.clone(),
                                            distance: 0,
                                        },
                                        SearcherState {
                                            valve: t0.to_string(),
                                            distance: *dist_memo
                                                .get(&(from_1.clone(), t0.to_string()))
                                                .unwrap(),
                                        },
                                    ],
                                    pressure: state.pressure,
                                    flow_rate: state.flow_rate,
                                    to_open: new_to_open.clone(),
                                });
                            } else {
                                let mut new_to_open = state.to_open.clone();
                                new_to_open.remove(*t0);
                                new_to_open.remove(*t1);

                                search.push(SearchState {
                                    time: state.time,
                                    searcher: [
                                        SearcherState {
                                            valve: t0.to_string(),
                                            distance: *dist_memo
                                                .get(&(from_0.clone(), t0.to_string()))
                                                .unwrap(),
                                        },
                                        SearcherState {
                                            valve: t1.to_string(),
                                            distance: *dist_memo
                                                .get(&(from_1.clone(), t1.to_string()))
                                                .unwrap(),
                                        },
                                    ],
                                    pressure: state.pressure,
                                    flow_rate: state.flow_rate,
                                    to_open: new_to_open,
                                });
                            }
                        }
                    }
                } else if target_list_0.is_empty() {
                    // Searcher 0 has no candidate target, send searcher 1
                    // to its targets.
                    for t1 in target_list_1 {
                        let mut new_to_open = state.to_open.clone();
                        new_to_open.remove(t1);

                        search.push(SearchState {
                            time: state.time,
                            searcher: [
                                SearcherState {
                                    valve: from_0.clone(),
                                    distance: 0,
                                },
                                SearcherState {
                                    valve: t1.clone(),
                                    distance: *dist_memo
                                        .get(&(from_1.clone(), t1.clone()))
                                        .unwrap(),
                                },
                            ],
                            pressure: state.pressure,
                            flow_rate: state.flow_rate,
                            to_open: new_to_open,
                        });
                    }
                } else if target_list_1.is_empty() {
                    // Searcher 1 has no candidate target, send searcher 0
                    // to its targets.
                    for t0 in target_list_0 {
                        let mut new_to_open = state.to_open.clone();
                        new_to_open.remove(t0);

                        search.push(SearchState {
                            time: state.time,
                            searcher: [
                                SearcherState {
                                    valve: t0.clone(),
                                    distance: *dist_memo
                                        .get(&(from_0.clone(), t0.clone()))
                                        .unwrap(),
                                },
                                SearcherState {
                                    valve: from_1.clone(),
                                    distance: 0,
                                },
                            ],
                            pressure: state.pressure,
                            flow_rate: state.flow_rate,
                            to_open: new_to_open,
                        });
                    }
                }
            }
        } else if state.searcher[0].distance > 0 && state.searcher[1].distance > 0 {
            // Both searchers are going to a target. Advance time until one or
            // both reach its target.
            let time_adv = cmp::min(state.searcher[0].distance, state.searcher[1].distance);
            let new_time = state.time + time_adv;
            if new_time < 26 {
                let mut new_flow_rate = state.flow_rate;
                for searcher in &state.searcher {
                    if searcher.distance == time_adv {
                        new_flow_rate += valves.get(&searcher.valve).unwrap().flow;
                    }
                }

                search.push(SearchState {
                    time: new_time,
                    searcher: [
                        SearcherState {
                            valve: state.searcher[0].valve.clone(),
                            distance: state.searcher[0].distance - time_adv,
                        },
                        SearcherState {
                            valve: state.searcher[1].valve.clone(),
                            distance: state.searcher[1].distance - time_adv,
                        },
                    ],
                    pressure: state.pressure + time_adv as u32 * state.flow_rate,
                    flow_rate: new_flow_rate,
                    to_open: state.to_open.clone(),
                });
            }
        } else {
            // One searcher needs a new target.
            let new_target_searcher_index = if state.searcher[0].distance == 0 {
                0
            } else {
                1
            };
            let new_target_searcher = &state.searcher[new_target_searcher_index];
            let other_searcher = &state.searcher[1 - new_target_searcher_index];

            let time_left = 25 - state.time as u8;
            let from = &new_target_searcher.valve;
            fill_dist_map(&mut dist_memo, &valves, &from, &state.to_open, time_left);

            let mut target_list: Vec<&String> = state
                .to_open
                .iter()
                .filter(|v| {
                    dist_memo
                        .get(&(from.clone(), v.to_string()))
                        .map_or(false, |&d| d < time_left)
                })
                .collect();
            target_list
                .sort_unstable_by_key(|v| dist_memo.get(&(from.clone(), v.to_string())).unwrap());

            if target_list.is_empty() {
                // No more viable targets to go to, just advance time until the
                // other searcher reaches its target.
                let time_adv = other_searcher.distance;
                let new_time = state.time + time_adv;
                if new_time <= 26 {
                    let new_flow_rate =
                        state.flow_rate + valves.get(&other_searcher.valve).unwrap().flow;
                    search.push(SearchState {
                        time: new_time,
                        searcher: [
                            SearcherState {
                                valve: state.searcher[0].valve.clone(),
                                distance: 0,
                            },
                            SearcherState {
                                valve: state.searcher[1].valve.clone(),
                                distance: 0,
                            },
                        ],
                        pressure: state.pressure + time_adv as u32 * state.flow_rate,
                        flow_rate: new_flow_rate,
                        to_open: state.to_open.clone(),
                    });
                }
            } else {
                for target in target_list {
                    let mut new_to_open = state.to_open.clone();
                    new_to_open.remove(target);

                    search.push(SearchState {
                        time: state.time,
                        searcher: [
                            SearcherState {
                                valve: target.clone(),
                                distance: *dist_memo.get(&(from.clone(), target.clone())).unwrap(),
                            },
                            SearcherState {
                                valve: other_searcher.valve.clone(),
                                distance: other_searcher.distance,
                            },
                        ],
                        pressure: state.pressure,
                        flow_rate: state.flow_rate,
                        to_open: new_to_open,
                    });
                }
            }
        }

        // Account for idle state from this position.
        let idle_min = 27 - state.time;
        let pressure = state.pressure + state.flow_rate * idle_min as u32;
        if pressure > max {
            max = pressure;
        }
    }

    println!("{}", max);
}

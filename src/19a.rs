use std::io;
use std::vec::Vec;

struct Blueprint {
    // Cost in ore to build an ore robot.
    ore_robot_cost: u16,
    // Cost in ore to build a clay robot.
    clay_robot_cost: u16,
    // Cost in (ore, clay) to build an obsidian robot.
    obsidian_robot_cost: (u16, u16),
    // Cost in (ore, obsidian) to build a geode robot.
    geode_robot_cost: (u16, u16),
}

impl Blueprint {
    fn parse(line: String) -> Blueprint {
        let s: Vec<u16> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        return Blueprint {
            ore_robot_cost: s[0],
            clay_robot_cost: s[1],
            obsidian_robot_cost: (s[2], s[3]),
            geode_robot_cost: (s[4], s[5]),
        };
    }
}

struct ResourceState {
    amount: u16,
    robots: u16,
    // Optimization: whether this state passed on creating a robot for this
    // resource and should not attempt to create a robot for this resource for
    // future iterations (to reduce redundant/suboptimal states).
    pass: bool,
}

impl ResourceState {
    fn increment(&mut self) {
        self.amount += self.robots;
    }

    fn no_pass(&self) -> ResourceState {
        return ResourceState {
            amount: self.amount,
            robots: self.robots,
            pass: false,
        };
    }
}

struct FactoryState {
    ore: ResourceState,
    clay: ResourceState,
    obsidian: ResourceState,
    geode: ResourceState,
}

fn max_geodes(blueprint: &Blueprint) -> u16 {
    let mut states = Vec::from([FactoryState {
        ore: ResourceState {
            amount: 0,
            robots: 1,
            pass: false,
        },
        clay: ResourceState {
            amount: 0,
            robots: 0,
            pass: false,
        },
        obsidian: ResourceState {
            amount: 0,
            robots: 0,
            pass: false,
        },
        geode: ResourceState {
            amount: 0,
            robots: 0,
            pass: false,
        },
    }]);

    let max_ore_cost = *[
        blueprint.ore_robot_cost,
        blueprint.clay_robot_cost,
        blueprint.obsidian_robot_cost.0,
        blueprint.geode_robot_cost.0,
    ]
    .iter()
    .max()
    .unwrap();

    for _ in 0..24 {
        let mut new_states = Vec::new();

        for state in &mut states {
            let mut build_ore_robot = false;
            let mut build_clay_robot = false;
            let mut build_obsidian_robot = false;
            let mut build_geode_robot = false;

            if state.ore.amount >= blueprint.geode_robot_cost.0
                && state.obsidian.amount >= blueprint.geode_robot_cost.1
            {
                build_geode_robot = true;
            } else {
                if !state.obsidian.pass
                    && state.obsidian.robots < blueprint.geode_robot_cost.1
                    && state.ore.amount >= blueprint.obsidian_robot_cost.0
                    && state.clay.amount >= blueprint.obsidian_robot_cost.1
                {
                    build_obsidian_robot = true;
                }
                if !state.clay.pass
                    && state.clay.robots < blueprint.obsidian_robot_cost.1
                    && state.ore.amount >= blueprint.clay_robot_cost
                {
                    build_clay_robot = true;
                }
                if !state.ore.pass
                    && state.ore.robots < max_ore_cost
                    && state.ore.amount >= blueprint.ore_robot_cost
                {
                    build_ore_robot = true;
                }
            }

            state.ore.increment();
            state.clay.increment();
            state.obsidian.increment();
            state.geode.increment();

            if build_ore_robot {
                state.ore.pass = true;
            }
            if build_clay_robot {
                state.clay.pass = true;
            }
            if build_obsidian_robot {
                state.obsidian.pass = true;
            }

            if build_ore_robot {
                new_states.push(FactoryState {
                    ore: ResourceState {
                        amount: state.ore.amount - blueprint.ore_robot_cost,
                        robots: state.ore.robots + 1,
                        pass: false,
                    },
                    clay: state.clay.no_pass(),
                    obsidian: state.obsidian.no_pass(),
                    geode: state.geode.no_pass(),
                });
            }

            if build_clay_robot {
                new_states.push(FactoryState {
                    ore: ResourceState {
                        amount: state.ore.amount - blueprint.clay_robot_cost,
                        robots: state.ore.robots,
                        pass: false,
                    },
                    clay: ResourceState {
                        amount: state.clay.amount,
                        robots: state.clay.robots + 1,
                        pass: false,
                    },
                    obsidian: state.obsidian.no_pass(),
                    geode: state.geode.no_pass(),
                });
                state.clay.pass = true;
            }

            if build_obsidian_robot {
                new_states.push(FactoryState {
                    ore: ResourceState {
                        amount: state.ore.amount - blueprint.obsidian_robot_cost.0,
                        robots: state.ore.robots,
                        pass: false,
                    },
                    clay: ResourceState {
                        amount: state.clay.amount - blueprint.obsidian_robot_cost.1,
                        robots: state.clay.robots,
                        pass: false,
                    },
                    obsidian: ResourceState {
                        amount: state.obsidian.amount,
                        robots: state.obsidian.robots + 1,
                        pass: false,
                    },
                    geode: state.geode.no_pass(),
                });
            }

            if build_geode_robot {
                state.ore.amount -= blueprint.geode_robot_cost.0;
                state.obsidian.amount -= blueprint.geode_robot_cost.1;
                state.geode.robots += 1;
            }
        }

        let mut i = 0;
        while i < states.len() {
            let state = &states[i];
            if state.ore.pass && state.clay.pass && state.obsidian.pass {
                states.remove(i);
            } else {
                i += 1;
            }
        }

        states.append(&mut new_states);
    }

    return states
        .iter()
        .map(|s| s.geode.amount)
        .max_by(|x, y| x.cmp(y))
        .unwrap();
}

fn main() {
    let blueprints: Vec<Blueprint> = io::stdin()
        .lines()
        .map(|l| Blueprint::parse(l.unwrap()))
        .collect();

    let quality: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| max_geodes(&b) as usize * (i + 1))
        .sum();
    println!("{}", quality);
}

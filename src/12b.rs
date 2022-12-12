use std::collections::{HashMap, HashSet};
use std::io;
use std::vec::Vec;

struct SearchState {
    loc: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl SearchState {
    fn new(loc: (i32, i32)) -> SearchState {
        SearchState {
            loc: loc,
            visited: HashSet::new(),
        }
    }

    fn copy_and_extend(&self, loc: (i32, i32)) -> SearchState {
        let mut new_visited = self.visited.clone();
        new_visited.insert(self.loc);
        return SearchState {
            loc: loc,
            visited: new_visited,
        };
    }

    fn contains(&self, loc: &(i32, i32)) -> bool {
        self.visited.contains(loc)
    }
}

fn find(map: &Vec<Vec<u8>>, target: u8) -> (i32, i32) {
    for (r, row) in map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == target {
                return (r as i32, c as i32);
            }
        }
    }
    panic!("map should have target '{}'", target);
}

fn get_elev(map: &Vec<Vec<u8>>, loc: &(i32, i32)) -> u8 {
    match map[loc.0 as usize][loc.1 as usize] {
        b'S' => b'a',
        b'E' => b'z',
        v => v,
    }
}

fn main() {
    // Parse input into matrix (vec of vec).
    let map: Vec<Vec<u8>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().bytes().collect::<Vec<u8>>())
        .collect();

    let end = find(&map, b'E');

    let mut search = Vec::new();

    for (r, row) in map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let elev = *cell;
            if elev == b'a' || elev == b'S' {
                search.push(SearchState::new((r as i32, c as i32)));
            }
        }
    }

    // Put closest starting point at the end so that it's popped first from the
    // search stack.
    search.sort_unstable_by_key(|s| {
        let (r, c) = s.loc;
        return map.len() + map[0].len() - (r.abs_diff(end.0) + c.abs_diff(end.1)) as usize;
    });

    let mut dists = HashMap::new();
    let mut min = None;

    while !search.is_empty() {
        let state = search
            .pop()
            .expect("search stack should not run out of states");

        let loc = state.loc;
        let len = state.visited.len();

        // If the path is already longer than the currently found minimum, then
        // stop searching. This is for optimization + accurate recording of the
        // answer.
        if min.map(|v| len >= v).unwrap_or(false) {
            continue;
        }

        // If the path to this location is already longer than a previous path,
        // then stop searching. This is for optimization.
        if dists.get(&loc).map(|&v| len >= v).unwrap_or(false) {
            continue;
        }

        // Record distance to this location.
        dists.insert(loc.clone(), len);

        // Use A*-esque search by always going towards end.
        let mut next_locs: Vec<(i32, i32)> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            // Find the possible frontier locations from this current location.
            .map(|d| (loc.0 as i32 + d.0, loc.1 as i32 + d.1))
            // Remove out-of-bounds locations.
            .filter(|(r, c)| {
                *r >= 0 && *r < map.len() as i32 && *c >= 0 && *c < map[0].len() as i32
            })
            // Remove locations that do not fulfill the elevation requirement.
            .filter(|new_loc| {
                let cur_elev = get_elev(&map, &loc);
                let to_elev = get_elev(&map, &new_loc);
                return to_elev <= cur_elev + 1;
            })
            // Remove locations which have already been visited on this path.
            .filter(|new_loc| !state.contains(&new_loc))
            .collect();
        // Sort search locations by distance to end location, in descending
        // order so that the closer locations are put at the end of the search
        // stack and thus are searched first when popped off the stack.
        next_locs.sort_unstable_by_key(|(r, c)| {
            map.len() + map[0].len() - (r.abs_diff(end.0) + c.abs_diff(end.1)) as usize
        });

        for new_loc in next_locs {
            if new_loc == end {
                min = Some(len);
            } else {
                search.push(state.copy_and_extend(new_loc));
            }
        }
    }

    println!("{}", min.expect("should have found solution") + 1);
}

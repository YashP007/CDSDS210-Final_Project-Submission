use std::collections::HashMap;

pub fn print_connections(map: &HashMap<String, Vec<String>>) {
    // Print out some connections
    for (artist, playlist) in map.iter() {
        println!("{:?} -> {:?}\n", artist, playlist);
    }
}

pub fn print_n_connections(map: &HashMap<String, Vec<String>>, mut n: i32) {
    print_information(map);
    // Print out some connections
    for (artist, playlist) in map.iter() {
        if n > 0 {
            println!("{:?} -> {:?}\n", artist, playlist);
        }
        n = n - 1;
    }
}

// Print Out Graph Charectaristics
fn print_information(map: &HashMap<String, Vec<String>>) {
    println!("Graph Informtion:");
    println!("\t\tSize of Hashmap: {:?}", map.len());

    println!();
}

// Recamending Optimal Connected Node (1 Edge Away)
pub fn one_node_away_recamendation(
    map: &HashMap<String, Vec<String>>,
    starting_node: &String,
) -> (Vec<String>, u32) {
    let connected_playlists = map.get(starting_node);

    let mut connections: HashMap<String, u32> = HashMap::new();
    let mut max_count: u32 = 0 as u32;
    let mut recamended_playlists: Vec<String> = Vec::new();

    if (connected_playlists.is_some()) {
        for playlist in connected_playlists.unwrap().iter() {
            let count: u32 = map.get(playlist).iter().len() as u32;
            let _ret = connections.insert(playlist.to_string(), count);
            if count > max_count {
                max_count = count;
            }
        }
        for (playlist, count) in connections.iter() {
            if count == &max_count {
                recamended_playlists.push(playlist.to_string());
            }
        }
    } else {
        recamended_playlists.push("No Playlists Recamended".to_string());
        max_count = 0;
    }
    return (recamended_playlists, max_count);
}

// Count Number of Cycles using depth first search
//fix: redo count_cycles algorithm such that it works & does not get stuck in a infinite loop at line 76
pub fn count_cycles(adjacency_list: &HashMap<String, Vec<String>>) -> u32 {
    //let mut visited = vec![false; adjacency_list.len()];
    let mut visited: Vec<String> = Vec::new();
    let mut stack = Vec::new();
    let mut cycles = 0;

    for (node, neighbors) in adjacency_list {
        if !visited.contains(node) {
            stack.push(node.to_string());
            visited.push(node.to_string());
            println!("In Position 1");

            println!("Stcak: {:?}", stack);

            while !stack.is_empty() {
                println!("In Position 2");
                let current_node = stack.pop().unwrap();
                for neighbor in neighbors {
                    println!("In Position 3 {:?}", neighbor);
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.to_string());
                        visited.push(node.to_string());
                    } else {
                        cycles += 1;
                    }
                }
            }
        }
    }

    cycles
}

// Count Trianlges Function
pub fn count_triangles(adjacency_list: HashMap<String, Vec<String>>) -> usize {
    let mut triangle_count = 0;

    for (node, neighbors) in &adjacency_list {
        for neighbor in neighbors {
            if let Some(neighbor_neighbors) = adjacency_list.get(neighbor) {
                for neighbor_neighbor in neighbor_neighbors {
                    if node != neighbor_neighbor && adjacency_list[node].contains(neighbor_neighbor)
                    {
                        triangle_count += 1;
                    }
                }
            }
        }
    }

    return triangle_count;
}

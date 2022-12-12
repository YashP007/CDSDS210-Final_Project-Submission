/* Yash Patel
    CDS DS210: Final Project
    Spotify Playlist Recamendation Graph Algorithm
    DUE: DEC 15, 2022
*/
pub mod graph_feats;
pub mod make_graph;
pub mod tests;
//use std::collections::HashMap;
//use std::error::Error;

fn main() {
    // Make Hash Map 1
    let file_path = "small_sample.csv";
    //let file_path = "sampled_split.csv";
    let hash_map = make_graph::make_hashmap(&file_path).unwrap();
    let hash_map_reversed = make_graph::make_rev_hashmap(&file_path).unwrap();

    // indicate inital hash_map creation was sucessful
    print(
        "Inital Hash Maps Creates",
        "Playlist -> Aritst && Artist -> Playlist",
    );

    print("Playlist -> Artist Map", " ");
    graph_feats::print_n_connections(&hash_map, 3);
    print("Artist -> Playlist Map", " ");
    graph_feats::print_n_connections(&hash_map_reversed, 3);

    // Make Secondary Hash Map
    let adj_list = make_graph::hash_map_playlist_playlist(&hash_map, &hash_map_reversed);

    print("Playlist -> Playlist Map (Adj List)", " ");
    graph_feats::print_n_connections(&adj_list, 3);

    // Execute Recamendation Algorithm
    let liked_playlist = "soul sister".to_string(); // this is the starting node of the graph
    print("One Node Away Recamendation", &" ".to_string());
    let recamendation = graph_feats::one_node_away_recamendation(&adj_list, &liked_playlist);
    println!(
        "{:?} --> Recomended {:?}\n\t\tWith Count {:?}",
        liked_playlist, recamendation.0, recamendation.1
    );

    // Count Number of Cycles in the Graph
    // To many cycles in graph to count, does not yeild usefull information about the dataset.
    /*
    let value = format!("HashMap Size: {:?}", &adj_list.len().to_string());
    print("Counting Number of Cycles in the Graph", &value);
    let num_cycles = graph_feats::count_cycles(&adj_list);
    println!("Number of Cycles Present in Graph: {:?}", num_cycles);
    */
}

fn print(title: &str, value: &str) {
    println!("\n\t{:?}", title);
    println!("----------------------------");
    println!("{:?}", value);
}

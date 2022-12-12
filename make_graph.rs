use csv;
use std::collections::HashMap;
use std::error::Error;

pub fn make_hashmap(file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut playlist_artist: HashMap<String, Vec<String>> = HashMap::new();
    let mut reader = csv::Reader::from_path(file_path)?;
    let _headers = reader.headers()?;
    for result in reader.records() {
        let record = result?;
        //let sucess = playlist_artist.insert(record[1].to_string(), record[0].to_string());
        playlist_artist
            .entry(record[1].to_string())
            .or_default()
            .push(record[0].to_string());
    }
    return Ok(playlist_artist);
}

pub fn make_rev_hashmap(file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut playlist_artist: HashMap<String, Vec<String>> = HashMap::new();
    let mut reader = csv::Reader::from_path(file_path)?;
    let _headers = reader.headers()?;
    for result in reader.records() {
        let record = result?;
        //let sucess = playlist_artist.insert(record[1].to_string(), record[0].to_string());
        playlist_artist
            .entry(record[0].to_string())
            .or_default()
            .push(record[1].to_string());
    }
    return Ok(playlist_artist);
}

// get_new_graph function
pub fn hash_map_playlist_playlist(
    playlist_map: &HashMap<String, Vec<String>>,
    artist_map: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

    for (playlist, artist) in playlist_map.iter() {
        // get list of all other connected nodes
        let connected_node_list = get_connected_node(&artist_map, artist);
        adj_list.insert(playlist.to_string(), connected_node_list);
    }

    return adj_list;
}

// get_connected_node function
fn get_connected_node(
    artist_map: &HashMap<String, Vec<String>>,
    artists: &Vec<String>,
) -> Vec<String> {
    let mut connected_nodes: Vec<String> = Vec::new();

    for artist in artists.iter() {
        //print!("Artist: {:?}", artist);
        //println!("\tPlaylists: {:?}", artist_map.get(artist).unwrap());
        let playlist_connections = artist_map.get(artist).unwrap();
        for value in playlist_connections {
            if !connected_nodes.contains(value) {
                connected_nodes.push(value.to_string());
            }
        }
    }
    return connected_nodes;
}

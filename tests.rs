#[cfg(test)]
#[path = "./graph_feats.rs"]
mod graph_feats;
use std::collections::HashMap;

/*
#[test]
fn check_count_cycles() {
    let adjacency_list: HashMap<String, Vec<String>> = vec![
        ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
        ("B".to_string(), vec!["C".to_string()]),
        ("C".to_string(), vec!["A".to_string()]),
    ]
    .into_iter()
    .collect();

    let num_cycles = graph_feats::count_cycles(&adjacency_list);
}
*/

#[test]
fn check_num_triganles() {
    let adjacency_list_1: HashMap<String, Vec<String>> = vec![
        ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
        ("B".to_string(), vec!["C".to_string()]),
        ("C".to_string(), vec!["A".to_string()]),
    ]
    .into_iter()
    .collect();

    let adjacency_list_2: HashMap<String, Vec<String>> = vec![
        (
            "A".to_string(),
            vec!["B".to_string(), "D".to_string(), "E".to_string()],
        ),
        ("B".to_string(), vec!["D".to_string()]),
        ("C".to_string(), vec!["C".to_string(), "F".to_string()]),
        (
            "D".to_string(),
            vec!["A".to_string(), "C".to_string(), "F".to_string()],
        ),
        ("E".to_string(), vec!["C".to_string(), "F".to_string()]),
        ("F".to_string(), vec!["B".to_string(), "E".to_string()]),
    ]
    .into_iter()
    .collect();
    #[cfg(test)]
    #[path = "./graph_feats.rs"]
    mod graph_feats;
    use std::collections::HashMap;

    /*
    #[test]
    fn check_count_cycles() {
        let adjacency_list: HashMap<String, Vec<String>> = vec![
            ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
            ("B".to_string(), vec!["C".to_string()]),
            ("C".to_string(), vec!["A".to_string()]),
        ]
        .into_iter()
        .collect();

        let num_cycles = graph_feats::count_cycles(&adjacency_list);
    }
    */

    #[test]
    fn check_num_triganles() {
        let adjacency_list_1: HashMap<String, Vec<String>> = vec![
            ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
            ("B".to_string(), vec!["C".to_string()]),
            ("C".to_string(), vec!["A".to_string()]),
        ]
        .into_iter()
        .collect();

        let adjacency_list_2: HashMap<String, Vec<String>> = vec![
            (
                "A".to_string(),
                vec!["B".to_string(), "D".to_string(), "E".to_string()],
            ),
            ("B".to_string(), vec!["D".to_string()]),
            ("C".to_string(), vec!["C".to_string(), "F".to_string()]),
            (
                "D".to_string(),
                vec!["A".to_string(), "C".to_string(), "F".to_string()],
            ),
            ("E".to_string(), vec!["C".to_string(), "F".to_string()]),
            ("F".to_string(), vec!["B".to_string(), "E".to_string()]),
        ]
        .into_iter()
        .collect();

        let num_cycles_1 = graph_feats::count_triangles(adjacency_list_1);
        let num_cycles_2 = graph_feats::count_triangles(adjacency_list_2);

        assert_eq!(num_cycles_1, 1 as usize);
        assert_eq!(num_cycles_2, 6 as usize);
    }

    #[test]
    fn check_recamendation() {
        let adjacency_list_2: HashMap<String, Vec<String>> = vec![
            (
                "A".to_string(),
                vec!["B".to_string(), "D".to_string(), "E".to_string()],
            ),
            ("B".to_string(), vec!["D".to_string()]),
            ("C".to_string(), vec!["C".to_string(), "F".to_string()]),
            (
                "D".to_string(),
                vec!["A".to_string(), "C".to_string(), "F".to_string()],
            ),
            ("E".to_string(), vec!["C".to_string(), "F".to_string()]),
            ("F".to_string(), vec!["B".to_string(), "E".to_string()]),
        ]
        .into_iter()
        .collect();

        let startPos = "A".to_string();
        let rec_ret = graph_feats::one_node_away_recamendation(&adjacency_list_2, &startPos);
        let recs = rec_ret.0;

        assert_eq!(recs.contains(&"D".to_string()), true);
        assert_eq!(recs.contains(&"B".to_string()), true);
        assert_eq!(recs.contains(&"E".to_string()), true);
    }

    let num_cycles_1 = graph_feats::count_triangles(adjacency_list_1);
    let num_cycles_2 = graph_feats::count_triangles(adjacency_list_2);

    assert_eq!(num_cycles_1, 1 as usize);
    assert_eq!(num_cycles_2, 6 as usize);
}

#[test]
fn check_recamendation() {
    let adjacency_list_2: HashMap<String, Vec<String>> = vec![
        (
            "A".to_string(),
            vec!["B".to_string(), "D".to_string(), "E".to_string()],
        ),
        ("B".to_string(), vec!["D".to_string()]),
        ("C".to_string(), vec!["C".to_string(), "F".to_string()]),
        (
            "D".to_string(),
            vec!["A".to_string(), "C".to_string(), "F".to_string()],
        ),
        ("E".to_string(), vec!["C".to_string(), "F".to_string()]),
        ("F".to_string(), vec!["B".to_string(), "E".to_string()]),
    ]
    .into_iter()
    .collect();

    let startPos = "A".to_string();
    let rec_ret = graph_feats::one_node_away_recamendation(&adjacency_list_2, &startPos);
    let recs = rec_ret.0;

    assert_eq!(recs.contains(&"D".to_string()), true);
    assert_eq!(recs.contains(&"B".to_string()), true);
    assert_eq!(recs.contains(&"E".to_string()), true);
}

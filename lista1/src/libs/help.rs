use std::{fs, process};

use graph_tsp::graph::Graph;

pub fn read_file(path: &String) -> Graph {
    let content: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1)
        }
    };
    Graph::create_graph(&content)
}

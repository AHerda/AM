use std::{fs, process};

use graph_tsp::graph::Graph;

pub fn read_file(path: &str) -> Graph {
    let content: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1)
        }
    };
    Graph::create_graph(&content)
}

pub fn read_dir(dir_path: &str) -> Vec<Graph> {
    let files = fs::read_dir(dir_path).unwrap();

    files
        .map(|file| {
            let file_str = file.unwrap().path().to_str().unwrap().to_string();
            read_file(&file_str)
        })
        .collect()
}

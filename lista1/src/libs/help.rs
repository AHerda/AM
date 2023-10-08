use std::{fs, process};

use super::vertecies::{Graph, Point};

pub fn read_file(path: &String) -> Graph {
    let content: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1)
        }
    };
    create_graph(&content)
}

pub fn create_graph(content: &str) -> Graph {
    let mut graph: Graph = Graph::new();
    let mut flag: bool = false;
    content.split("\r\n").for_each(|line| {
        if flag {
            let numbers: Vec<&str> = line.split(' ').collect();
            if numbers.len() == 3 {
                graph.add_point(Point::new(
                    numbers[0].parse::<i32>().unwrap(),
                    numbers[1].parse::<i32>().unwrap(),
                    numbers[2].parse::<i32>().unwrap(),
                ));
            } else {
                flag = false;
            }
        } else if line == "NODE_COORD_SECTION" {
            flag = true;
        } else if line.contains("DIMENSION") {
            let n: i32 = line.split(' ').nth(2).unwrap().parse::<i32>().unwrap();
            graph.set_size(n);
        }
    });

    graph
}

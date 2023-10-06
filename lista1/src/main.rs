use std::{env::args, process, fs};

use vertecies::{Graph, Point};

mod vertecies;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments");
        process::exit(1);
    }

    let mut graph = read_file(&args[1]);
    let mst = graph.mst();
    println!("{:#?}", mst);
}


fn read_file(path: &String) -> Graph {
    let content: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        },
    };
    create_graph(&content)
}

fn create_graph(content: &String) -> Graph {
    let mut graph: Graph = Graph::new();
    let mut flag: bool = false;
    content.split('\n').for_each(|line| {
        if flag {
            let numbers: Vec<&str> = line.split(' ').collect();
            graph.add_point(Point::new(
                numbers[0].parse::<i32>().unwrap(),
                numbers[1].parse::<i32>().unwrap(), 
                numbers[2].parse::<i32>().unwrap()
            ));
        }
        else if line == "NODE_COORD_SECTION" {
            flag = true;
        }
    });

    graph
}
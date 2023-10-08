use std::{env::args, process};

use crate::libs::{help::read_file, vertecies::Graph};

mod libs;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments");
        process::exit(1);
    }

    let mut graph = read_file(&args[1]);
    zadanie1(&mut graph);
    zadanie2(&graph);
    zadanie3(&graph);
}

fn zadanie1(graph: &mut Graph) {
    let size_mst = graph.mst();
    let mut sum = 0;
    graph.get_mst().clone().unwrap().iter().for_each(|x| sum += x.len());
    println!(
        "Minimum Spanning Tree:\n{:?}\n\n# of edges: {}\n\nSize of minimum spanning tree: {size_mst:^20}\n",
        graph.get_mst().clone().unwrap(),
        sum / 2
    );
}

fn zadanie2(graph: &Graph) {
    let (mst_dfs_path, times_visited, size_mst_dfs) = graph.dfs().unwrap();
    println!(
        "Path:\n{:?}\n\nSize of path: {size_mst_dfs:^20}\n\nHow many times each node is visited: {:?}",
        mst_dfs_path, times_visited
    );
}

fn zadanie3(graph: &Graph) {
    let mut avg_a = 0;
    let mut avg_b = 0;

    let mut min_c = i32::MAX;
    for _c in 0..20 {
        let mut min_b = i32::MAX;
        for _b in 0..5 {
            let mut min_a = i32::MAX;
            for _a in 0..10{
                let (_path, size) = graph.random_path();
                if size < min_a {
                    min_a = size;
                    if size < min_b {
                        min_b = size;
                        if size < min_c { min_c = size; }
                    }
                }
            }
            avg_a += min_a;
        }
        avg_b += min_b;
    }

    avg_a /= 100;
    avg_b /= 20;

    println!("|{:^20}|{:^20}|{:^20}|", "min_c", "avg_b", "avg_a");
    println!("|{:^20}|{:^20}|{:^20}|", min_c, avg_b, avg_a);
}
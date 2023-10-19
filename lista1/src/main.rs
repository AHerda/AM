use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

use crate::libs::{help::read_file, vertecies::Graph};

mod libs;

fn main() {
    // let args: Vec<String> = args().collect();
    // if args.len() != 2 {
    //     eprintln!("Wrong number of arguments");
    //     process::exit(1);
    // }

    // let mut graph = read_file(&args[1]);
    // zadanie1(&mut graph, true);
    // zadanie2(&graph, true);
    // zadanie3(&graph, true);

    wszystkie_zadania();
}

fn zadanie1(graph: &mut Graph, draw: bool, draw_full: bool, file: Option<&mut File>) -> u32 {
    let size_mst: u32 = graph.mst();
    let mut sum = 0;
    graph
        .get_mst()
        .clone()
        .unwrap()
        .iter()
        .for_each(|x| sum += x.len());

    if draw && draw_full {
        println!(
            "Minimum Spanning Tree:\n{:?}\n\n# of edges: {}\n\nSize of minimum spanning tree: {size_mst:^20}\n",
            graph.get_mst().clone().unwrap(),
            sum / 2
        );
    } else if draw {
        print!("{}", size_mst);
    }
    if let Some(file) = file {
        let mst = graph.get_mst().clone().unwrap();
        _ = write!(
            file,
            "{}",
            format!(
                "{:#?}\n",
                mst.iter()
                    .map(|v| v.iter().map(|(node, _weight)| *node).collect::<Vec<i32>>())
                    .collect::<Vec<Vec<i32>>>()
            )
            .replace(",\n    ]", "\n    ]")
            .replace(",\n]", "\n]")
        );
    }

    size_mst
}

fn zadanie2(graph: &Graph, draw: bool, draw_full: bool, file: Option<&mut File>) -> u32 {
    let (mst_dfs_path, times_visited, size_mst_dfs) = graph.dfs().unwrap();
    if draw && draw_full {
        println!(
            "Path:\n{:?}\n\nSize of path: {size_mst_dfs:^20}\n\nHow many times each node is visited: {:?}",
            mst_dfs_path, times_visited
        );
    } else if draw {
        print!("{}", size_mst_dfs);
    }
    if let Some(file) = file {
        _ = write!(
            file,
            "{}",
            format!("{:#?}", mst_dfs_path).replace(",\n]", "\n]")
        );
    }

    size_mst_dfs as u32
}

fn zadanie3(graph: &Graph, draw: bool, draw_full: bool) -> (u32, u32, u32) {
    let mut avg_a = 0;
    let mut avg_b = 0;

    let mut min_c = i32::MAX;
    for _c in 0..20 {
        let mut min_b = i32::MAX;
        for _b in 0..5 {
            let mut min_a = i32::MAX;
            for _a in 0..10 {
                let (_path, size) = graph.random_path();
                if size < min_a {
                    min_a = size;
                    if size < min_b {
                        min_b = size;
                        if size < min_c {
                            min_c = size;
                        }
                    }
                }
            }
            avg_a += min_a;
        }
        avg_b += min_b;
    }

    avg_a /= 100;
    avg_b /= 20;

    if draw && draw_full {
        println!("|{:^20}|{:^20}|{:^20}|", "min_c", "avg_b", "avg_a");
        println!("|{:^20}|{:^20}|{:^20}|", min_c, avg_b, avg_a);
    } else if draw {
        print!("{};{};{}", min_c, avg_b, avg_a);
    }

    (min_c as u32, avg_b as u32, avg_a as u32)
}

fn wszystkie_zadania() {
    let files = fs::read_dir(r".\data").unwrap();
    let mut out_file = File::open("csv/dane.csv").expect("Nie ma pliku");
    files.for_each(|file| {
        let file_str = file.unwrap().path().to_str().unwrap().to_string();
        let mut graph_file =
            File::create(format!("help/graph_{}.json", &file_str[7..13])).expect("Nie ma pliku");
        let mut results_mst =
            File::create(format!("help/mst_{}.json", &file_str[7..13])).expect("Nie ma pliku");
        let mut results_dfs =
            File::create(format!("help/dfs_{}.json", &file_str[7..13])).expect("Nie ma pliku");

        let mut graph = read_file(&file_str);
        _ = write!(graph_file, "{}", graph.get_points_json());

        let mst = zadanie1(&mut graph, false, false, Some(&mut results_mst));
        let dfs = zadanie2(&graph, false, false, Some(&mut results_dfs));
        let (c, b, a) = zadanie3(&graph, false, false);

        _ = writeln!(
            out_file,
            "{};{};{};{};{};{}",
            &file_str[7..],
            mst,
            dfs,
            c,
            b,
            a
        );
    });

    Command::new("python")
        .arg("src/wykresy.py")
        .spawn()
        .expect("wykresy.py command failed to start");
}

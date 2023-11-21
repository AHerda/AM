mod libs;

use std::{fs::File, io::Write};

use graph_tsp::graph::Graph;
use libs::{graph_ext::Lista2, help};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

fn main() {
    let mut out = File::create("results/res.csv").expect("??!!");
    let mut v: Vec<Graph> = help::read_dir("./data");
    _ = writeln!(out, "graph_size;mst_size;avg_size;avg_steps;min_size");

    v.par_iter_mut().for_each(|g| {
        let mst_size = g.mst();
        let mut min_size = i32::MAX;
        let mut avg_size = 0;
        let mut avg_it = 0;
        let n_sqrt = (g.size() as f64).sqrt();
        for _i in 0..=(n_sqrt.floor() as i32) {
            let (mut path, _size) = g.dfs_random_start().expect("???!!!");
            let (ls_size, ls_it) = g.local_search(&mut path);

            avg_size += ls_size;
            avg_it += ls_it;

            if ls_size < min_size {
                min_size = ls_size;
            }

            println!("size: {}\n\tit: {}\n", n_sqrt, _i);
        }

        _ = writeln!(
            &out,
            "{};{};{};{};{}",
            g.size(),
            mst_size,
            avg_size as f64 / n_sqrt,
            avg_it as f64 / n_sqrt,
            min_size
        );
    });
}

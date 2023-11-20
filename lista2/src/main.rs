mod libs;

use graph_tsp::graph::Graph;
use libs::{help, graph_ext::Lista2};
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

fn main() {
    let mut v: Vec<Graph> = help::read_dir("./data");

    v.par_iter_mut().for_each(|g| { g.mst(); });
    // v.sort_by(|g1, g2| {g1.size().cmp(&g2.size())});

    v.par_iter().for_each(|g| {
        let (path, size) = g.dfs_random_start().expect("???!!!");
        println!("nodes = {}\n\tsize = {}", g.size(), size);
    });
}

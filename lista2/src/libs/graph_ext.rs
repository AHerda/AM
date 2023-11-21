use graph_tsp::graph::Graph;
use rand::{Rng, seq::SliceRandom};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait Lista2 {
    fn dfs_random_start(&self) -> Option<(Vec<i32>, i32)>;
    fn invert(&self, path: &mut [i32], from: i32, to: i32, neighbors: &Vec<Vec<i32>>) -> i32;
    fn measure_path(&self, path: &[i32], neighbors: &Vec<Vec<i32>>) -> i32;
    fn all_pairs(&self) -> Vec<(i32, i32)>;
    fn best_pair(&self, path: &mut Vec<i32>, all_pairs: &mut Vec<(i32, i32)>, neighbors: &Vec<Vec<i32>>) -> ((i32, i32), i32);
    fn local_search(&self, path: &mut Vec<i32>) -> (i32, i32);
}

impl Lista2 for Graph {
    fn dfs_random_start(&self) -> Option<(Vec<i32>, i32)> {
        let mut rng = rand::thread_rng();
        match &self.get_mst() {
            Some(_mst) => {
                let mut path: Vec<i32> = Vec::new();
                let mut size: i32 = 0;
                let mut visited: Vec<bool> = vec![false; (self.size() + 1) as usize];
                let mut number_visited = 0;
                let mut times_visited: Vec<i32> = vec![0; (self.size() + 1) as usize];
                _ = self.dfs_traverse(
                    None,
                    (rng.gen::<u32>() % self.size() as u32 + 1) as i32,
                    &mut visited,
                    &mut number_visited,
                    &mut path,
                    &mut size,
                    &mut times_visited,
                );
                Some((path, size))
            }
            None => None,
        }
    }

    fn local_search(&self, path: &mut Vec<i32>) -> (i32, i32) {
        let len = path.len();
        let neighbors = self.get_neighbors2();
        let mut it = 0;
        let mut all_pairs = self.all_pairs();
        let mut best_path_size = self.measure_path(path, &neighbors);
        loop {
            let (best_pair, size) = self.best_pair(path, &mut all_pairs, &neighbors);
            if size >= best_path_size || (it > 100 && len > 1000) {
                return (best_path_size, it);
            }
            best_path_size = size;
            path.swap(best_pair.0 as usize, best_pair.1 as usize);
            it += 1;
        }
    }

    fn all_pairs(&self) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        let n = self.size();

        for i in 0..n {
            for j in (i + 1)..n {
                result.push((i, j));
            }
        }
        result
    }

    fn best_pair(&self, path: &mut Vec<i32>, all_pairs: &mut Vec<(i32, i32)>, neighbors: &Vec<Vec<i32>>) -> ((i32, i32), i32) {
        let mut min = ((0, 0), i32::MAX);
        let mut rng = rand::thread_rng();
        all_pairs.shuffle(&mut rng);
        all_pairs[0..std::cmp::min(50*path.len(), all_pairs.len())]
            .iter()
            .for_each(|(a, b)| {
                let temp = self.invert(path, *a, *b, neighbors);
                if temp < min.1 {
                    min = ((*a, *b), temp);
                }
            });
        min
    }

    fn invert(&self, path: &mut [i32], from_id: i32, to_id: i32, neighbors: &Vec<Vec<i32>>) -> i32 {
        path.swap(from_id as usize, to_id as usize);
        let path_size = self.measure_path(&path, neighbors);
        path.swap(to_id as usize, from_id as usize);
        path_size
    }

    fn measure_path(&self, path: &[i32], neighbors: &Vec<Vec<i32>>) -> i32 {
        path.iter()
            .zip(path.iter().cycle().skip(1))
            .map(|(&from, &to)| neighbors[from as usize][to as usize])
            .sum()
    }
}

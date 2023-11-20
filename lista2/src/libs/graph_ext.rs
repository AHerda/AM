use graph_tsp::graph::Graph;
use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait Lista2 {
    fn dfs_random_start(&self) -> Option<(Vec<i32>, i32)>;
    fn invert(&self, path: &mut [i32], size: &mut i32, from: i32, to: i32);
    fn measure_path(&self, path: &[i32]) -> i32;
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

    fn invert(&self, path: &mut [i32], size: &mut i32, from_id: i32, to_id: i32) {
        let temp = path[from_id as usize];
        path[from_id as usize] = path[to_id as usize];
        path[to_id as usize] = temp;

        *size = self.measure_path(&path);
    }

    fn measure_path(&self, path: &[i32]) -> i32 {
        let mut size = 0;
        let len = path.len();

        for i in 0..len {
            size += self.get_neighbors()[path[i] as usize]
                .par_iter()
                .find_first(|e| e.to == path[(i + 1) % len])
                .expect("No edge?!")
                .weight;
        }

        size
    }
}

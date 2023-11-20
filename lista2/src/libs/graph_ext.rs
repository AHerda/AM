use graph_tsp::graph::Graph;
use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait Lista2 {
    fn dfs_random_start(&self) -> Option<(Vec<i32>, i32)>;
    fn invert(&self, path: &mut [i32], from: i32, to: i32) -> i32;
    fn measure_path(&self, path: &[i32]) -> i32;
    fn all_pairs(&self) -> Vec<(i32, i32)>;
    fn best_pair(&self, path: &Vec<i32>, all_pairs: &mut Vec<(i32, i32)>) -> ((i32, i32), i32);
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
                println!("Dupa_dfs");
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
        println!("Dupa local");
        let mut it = 0;
        let mut all_pairs = self.all_pairs();
        let mut best_path_size = self.measure_path(path);
        println!("Dupa local 2");
        loop {
            println!("it: {}\n\tsize: {}", it, best_path_size);
            let (best_pair, size) = self.best_pair(&path.clone(), &mut all_pairs);
            if size >= best_path_size {
                return (best_path_size, it);
            }
            best_path_size = size;
            self.invert(path, best_pair.0, best_pair.1);
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

    fn best_pair(&self, path: &Vec<i32>, all_pairs: &mut Vec<(i32, i32)>) -> ((i32, i32), i32) {
        println!("Dupa best pair\t{}", all_pairs.len());
        let all_pairs2 = all_pairs
            .par_iter()
            .map(|(a, b)| ((*a, *b), self.invert(&mut path.clone(), *a, *b)))
            .collect::<Vec<((i32, i32), i32)>>();
        println!("Dupa best pair2");
        *all_pairs2
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .expect("Da_fuq?!?")
    }

    fn invert(&self, path: &mut [i32], from_id: i32, to_id: i32) -> i32 {
        path.swap(from_id as usize, to_id as usize);
        self.measure_path(&path)
    }

    fn measure_path(&self, path: &[i32]) -> i32 {
        let mut size = 0;
        let len = path.len();
        let neighbors = self.get_neighbors2();

        for i in 0..len {
            size += neighbors[path[i] as usize][path[(i + 1) % len] as usize];
        }

        size
    }
}

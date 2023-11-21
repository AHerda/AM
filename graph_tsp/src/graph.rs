use rand::{
    prelude::SliceRandom,
    thread_rng
};

use crate::edge::Edge;
use crate::point::Point;

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Edge>,
    neighbors: Vec<Vec<Edge>>,
    neighbors2: Vec<Vec<i32>>,
    points: Vec<Point>,
    n: i32,
    mst: Option<Vec<Vec<(i32, i32)>>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            edges: Vec::new(),
            neighbors: Vec::new(),
            neighbors2: Vec::new(),
            points: Vec::new(),
            n: 0,
            mst: None,
        }
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

    pub fn print(&self) {
        self.points.iter().for_each(|point| println!("{:?}", point))
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.iter().for_each(|point_temp| {
            let temp_edge: Edge = Edge::new(&point, point_temp);
            let temp_reverse: Edge = Edge::new(point_temp, &point);
            let weight = temp_edge.weight;

            self.edges.push(temp_edge.clone());
            self.neighbors[point.id as usize].push(temp_edge);
            self.neighbors2[point.id as usize][point_temp.id as usize] = weight;

            self.edges.push(temp_reverse.clone()); // Can you remove this line?? idk
            self.neighbors[point_temp.id as usize].push(temp_reverse); // Can you remove this line?? idk
            self.neighbors2[point_temp.id as usize][point.id as usize] = weight; // Can you remove this line?? idk
        });

        self.points.push(point);
    }

    pub fn size(&self) -> i32 {
        self.n
    }

    pub fn set_size(&mut self, n: i32) {
        self.n = n;
        self.neighbors = vec![Vec::new(); (n + 1) as usize];
        self.neighbors2 = vec![vec![-1; (n + 1) as usize]; (n + 1) as usize];
    }

    pub fn edges(&self) -> i32 {
        (self.n * self.n - self.n) / 2
    }

    pub fn sort(&mut self) {
        self.edges.sort();
    }

    pub fn get_points(&self) -> Vec<Point> {
        self.points.clone()
    }

    pub fn get_points_json(&self) -> String {
        serde_json::to_string(&self.points).unwrap()
    }

    pub fn get_neighbors(&self) -> Vec<Vec<Edge>> {
        self.neighbors.clone()
    }

    pub fn get_neighbors2(&self) -> Vec<Vec<i32>> {
        self.neighbors2.clone()
    }

    pub fn get_mst(&self) -> &Option<Vec<Vec<(i32, i32)>>> {
        &self.mst
    }

    pub fn mst(&mut self) -> u32 {
        self.edges.sort();

        let mut mst = vec![Vec::new(); (self.n + 1) as usize];
        let mut size: u32 = 0;
        let mut visited: Vec<bool> = vec![false; (self.n + 1) as usize];

        visited[self.edges[0].from as usize] = true;

        while visited[1..].contains(&false) {
            let edge = self
                .edges
                .iter()
                .filter(|edge| {
                    (visited[edge.from as usize] && !visited[edge.to as usize])
                        || (!visited[edge.from as usize] && visited[edge.to as usize])
                })
                .nth(0)
                .unwrap();

            mst[edge.from as usize].push((edge.to, edge.weight));
            mst[edge.to as usize].push((edge.from, edge.weight));
            size += edge.weight as u32;

            visited[edge.from as usize] = true;
            visited[edge.to as usize] = true;
        }
        self.mst = Some(mst);
        size
    }

    pub fn dfs(&self) -> Option<(Vec<i32>, Vec<i32>, i32)> {
        match &self.mst {
            Some(_mst) => {
                let mut path: Vec<i32> = Vec::new();
                let mut size: i32 = 0;
                let mut visited: Vec<bool> = vec![false; (self.n + 1) as usize];
                let mut number_visited = 0;
                let mut times_visited: Vec<i32> = vec![0; (self.n + 1) as usize];

                _ = self.dfs_traverse(
                    None,
                    1,
                    &mut visited,
                    &mut number_visited,
                    &mut path,
                    &mut size,
                    &mut times_visited,
                );
                Some((path, times_visited, size))
            }
            None => None,
        }
    }

    pub fn dfs_traverse(
        &self,
        mut last: Option<i32>,
        current: i32,
        visited: &mut Vec<bool>,
        number_visited: &mut i32,
        path: &mut Vec<i32>,
        size: &mut i32,
        times_visited: &mut Vec<i32>,
    ) -> Option<i32> {
        let c: usize = current as usize;

        visited[c] = true;
        *number_visited += 1;
        times_visited[c] += 1;
        if let Some(last) = last {
            *size += self
                .edges
                .iter()
                .find(|&edge| edge.from == last && edge.to == current)
                .unwrap()
                .weight;
        }
        path.push(current);
        last = Some(current);

        self.mst.as_ref().unwrap()[c]
            .iter()
            .for_each(|&(neighbor, _weight)| {
                if !visited[neighbor as usize] {
                    last = self.dfs_traverse(
                        last,
                        neighbor,
                        visited,
                        number_visited,
                        path,
                        size,
                        times_visited,
                    );
                }
            });
        Some(current)
    }

    pub fn random_path(&self) -> (Vec<i32>, i32) {
        let mut table: Vec<i32> = (1..=self.n).collect();
        table.shuffle(&mut thread_rng());

        let mut size: i32 = 0;
        table.iter().enumerate().for_each(|(index, point)| {
            if index + 1 < table.len() {
                size += self.neighbors[*point as usize]
                    .iter()
                    .find(|x| x.to == table[index + 1])
                    .unwrap()
                    .weight;
            } else {
                size += self.neighbors[*point as usize]
                    .iter()
                    .find(|x| x.to == table[0])
                    .unwrap()
                    .weight;
            }
        });

        (table, size)
    }
}
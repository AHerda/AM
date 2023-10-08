use std::cmp::Ordering;
use rand::{thread_rng, prelude::SliceRandom};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub id: i32,
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(id: i32, x: i32, y: i32) -> Point {
        Point { id, x, y }
    }

    pub fn distance(&self, point: &Point) -> i32 {
        (((self.x - point.x).pow(2) + (self.y - point.y).pow(2)) as f64)
            .sqrt()
            .round() as i32
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge {
    from: i32,
    to: i32,
    weight: i32,
}

impl Edge {
    fn new(from: &Point, to: &Point) -> Edge {
        Edge {
            from: from.id,
            to: to.id,
            weight: from.distance(to),
        }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Graph {
    edges: Vec<Edge>,
    neighbors: Vec<Vec<Edge>>,
    points: Vec<Point>,
    n: i32,
    mst: Option<Vec<Vec<(i32, i32)>>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            edges: Vec::new(),
            neighbors: Vec::new(),
            points: Vec::new(),
            n: 0,
            mst: None,
        }
    }

    pub fn _print(&self) {
        self.points.iter().for_each(|point| println!("{:?}", point))
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.iter().for_each(|point_temp| {
            let temp_edge: Edge = Edge::new(&point, point_temp);
            let temp_reverse: Edge = Edge::new(point_temp, &point);

            self.edges.push(temp_edge.clone());
            self.neighbors[point.id as usize].push(temp_edge);

            self.edges.push(temp_reverse.clone()); // Can you remove this line?? idk
            self.neighbors[point_temp.id as usize].push(temp_reverse); // Can you remove this line?? idk
        });

        self.points.push(point);
    }

    pub fn _size(&self) -> i32 {
        self.n
    }

    pub fn set_size(&mut self, n: i32) {
        self.n = n;
        self.neighbors = vec![Vec::new(); (n + 1) as usize];
    }

    pub fn _edges(&self) -> i32 {
        (self.n * self.n - self.n) / 2
    }

    pub fn _sort(&mut self) {
        self.edges.sort();
    }

    pub fn get_mst(&self) -> &Option<Vec<Vec<(i32, i32)>>> {
        &self.mst
    }

    pub fn mst(&mut self) -> i32 {
        self.edges.sort();

        let mut mst = vec![Vec::new(); (self.n + 1) as usize];
        let mut size: i32 = 0;
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
            println!("{}", edge.weight);
            size += edge.weight;

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

                self.dfs_traverse(
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

    fn dfs_traverse(
        &self,
        current: i32,
        visited: &mut Vec<bool>,
        number_visited: &mut i32,
        path: &mut Vec<i32>,
        size: &mut i32,
        times_visited: &mut Vec<i32>,
    ) {
        let c: usize = current as usize;

        visited[c] = true;
        *number_visited += 1;
        times_visited[c] += 1;
        path.push(current);

        self.mst.as_ref().unwrap()[c]
            .iter()
            .for_each(|&(neighbor, weight)| {
                if !visited[neighbor as usize] {
                    *size += weight;
                    self.dfs_traverse(neighbor, visited, number_visited, path, size, times_visited);
                    if *number_visited != self.n {
                        path.push(current);
                        *size += weight;
                        times_visited[c] += 1;
                    }
                }
            });
    }

    pub fn random_path(&self) -> (Vec<i32>, i32) {
        let mut table: Vec<i32> = (1..=self.n).collect();
        table.shuffle(&mut thread_rng());

        let mut size: i32 = 0;
        table.iter().enumerate().for_each(|(index, point)| {
            if index + 1 < table.len() as usize {
                size += self.neighbors[*point as usize].iter().find(|x| x.to == table[index + 1]).unwrap().weight;
            }
            else {
                size += self.neighbors[*point as usize].iter().find(|x| x.to == table[0]).unwrap().weight;
            }
        });

        (table, size)
    }
}

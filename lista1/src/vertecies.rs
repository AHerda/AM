use std::cmp::Ordering;

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
        (((self.x - point.x).abs() + (self.y - point.y).abs()) as f64).sqrt().round() as i32
    }
}

impl<'a> Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.id.cmp(&self.id)
    }
}

impl<'a> PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    pub from: i32,
    pub to: i32,
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
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Graph {
    edges: Vec<Edge>,
    points: Vec<Point>,
    n: i32,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edges: Vec::new(), points: Vec::new(), n: 0 }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.iter().for_each(|point_temp| {
            self.edges.push(Edge::new(&point, point_temp));
        });


        self.points.push(point);
        self.n += 1;
    }

    pub fn _size(&self) -> i32 {
        self.n
    }

    pub fn _edges(&self) -> i32 {
        (self.n * self.n - self.n) / 2
    }

    pub fn _sort(&mut self) {
        self.edges.sort();
    }

    pub fn mst(&mut self) -> Vec<(i32, i32)> {
        self.edges.sort();

        let mut mst: Vec<(i32, i32)> = Vec::new();
        let mut disjoint_sets: Vec<i32> = (0..self.n).collect();

        for edge in &self.edges {
            if Self::find_set(edge.from, &mut disjoint_sets) != Self::find_set(edge.to, &mut disjoint_sets) {
                mst.push((edge.from, edge.to));
                Self::union_sets(edge.from, edge.to, &mut disjoint_sets);
            }
        }

        mst
    }

    fn find_set(x: i32, disjoint_sets: &mut Vec<i32>) -> i32 {
        let x2 = x as usize;
        if disjoint_sets[x2] != x {
            disjoint_sets[x2] = Self::find_set(disjoint_sets[x2], disjoint_sets);
        }
        disjoint_sets[x2]
    }
    
    fn union_sets(x: i32, y: i32, disjoint_sets: &mut Vec<i32>) {
        let root_x = Self::find_set(x, disjoint_sets);
        let root_y = Self::find_set(y, disjoint_sets);
        disjoint_sets[root_x as usize] = root_y;
    }
}
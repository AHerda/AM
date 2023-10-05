use std::cmp::Ordering;

#[derive(Debug)]
pub struct Point {
    id: i32,
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

#[derive(Debug, PartialEq, Eq)]
struct Edge<'a> {
    from: &'a Point,
    to: &'a Point,
    weight: i32,
}

impl<'a> Edge<'a> {
    fn new(from: &'a Point, to: &'a Point) -> Edge<'a> {
        Edge {
            from,
            to,
            weight: from.distance(to),
        }
    }
}

impl<'a> Ord for Edge<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl<'a> PartialOrd for Edge<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Graph<'a> {
    edges: Vec<Edge<'a>>,
    points: Vec<&'a Point>,
    n: i32,
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph { edges: Vec::new(), points: Vec::new(), n: 0 }
    }

    pub fn add_point(&mut self, point: &'a Point) {
        self.points.iter().for_each(|point_temp| {
            self.edges.push(Edge::new(point, point_temp));
        });
        self.points.push(point);
        self.n += 1;
    }

    pub fn size(&self) -> i32 {
        self.n
    }

    pub fn edges(&self) -> i32 {
        (self.n * self.n - self.n) / 2
    }

    pub fn sort(&mut self) {
        self.edges.sort();
    }

    pub fn minimum_sapnning_tree(&self) -> Vec<Edge> {
        self.edges.sort();

        let mut mst: Vec<Edge> = Vec::new();
        let mut disjoint_sets: Vec<i32> = (0..self.n).collect();

        for edge in self.edges {
            if Self::find_set(edge.from, &disjoint_sets) != Self::find_set(edge.to, &disjoint_sets) {
                mst.push(edge);
                Self::union_sets(edge.from, edge.to, &mut disjoint_sets);
            }
        }

        mst
    }
}
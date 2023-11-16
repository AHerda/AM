use std::cmp::Ordering;

use crate::point::Point;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge {
    pub from: i32,
    pub to: i32,
    pub weight: i32,
}

impl Edge {
    pub fn new(from: &Point, to: &Point) -> Edge {
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
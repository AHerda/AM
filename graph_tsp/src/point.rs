use serde::Serialize;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
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

    pub fn _get_coord(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn _to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
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
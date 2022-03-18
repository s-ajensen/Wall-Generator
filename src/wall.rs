use crate::point;

pub struct Wall {
    points: Vec<point::Point>,
    length: usize
}

impl Wall {
    pub fn new(points: Vec<point::Point>, length: usize) -> Wall {
        Wall {
            points: points,
            length: length
        }
    }
}
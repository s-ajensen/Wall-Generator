use crate::point;

pub struct Wall {
    pub points: Vec<point::Point>,
    pub length: usize
}

impl Wall {
    pub fn new(points: Vec<point::Point>, length: usize) -> Wall {
        Wall {
            points: points,
            length: length
        }
    }
}
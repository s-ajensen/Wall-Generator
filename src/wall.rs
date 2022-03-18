use crate::point;

pub struct Wall {
    pub points: Vec<point::Point>,
    pub length: usize,
    pub is_horizontal: bool,
    pub child: Option<Box<Wall>>
}

impl Wall {
    pub fn new(points: Vec<point::Point>, length: usize, is_horizontal: bool, wall: Option<Box<Wall>>) -> Wall {
        Wall {
            points: points,
            length: length,
            is_horizontal: is_horizontal,
            child: wall
        }
    }
}
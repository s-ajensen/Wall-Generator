use colored::Colorize;
use rand::Rng;
use crate::line;
use crate::point;
use crate::wall;

pub struct Lattice {
    width: usize,
    height: usize,
    pub data: Vec<Vec<char>>
}
    
impl Lattice {
    pub fn new(width: usize, height: usize) -> Lattice {
        Lattice {
            width: width,
            height: height,
            data: vec![vec!['0'; height]; width]
        }
    }

    pub fn print(&self) {
        for row in &self.data {
            for col in row {
                if format!("{}", col) == "0" {
                    print!("{} ", format!("0").blue())
                } else if format!("{}", col) == "#" {
                    print!("{} ", format!("#").red().bold())
                }
            }
            print!("\n");
        }
    }

    pub fn draw_line(&self, root: &point::Point, is_horizontal: bool, length: usize) -> Vec<point::Point> {
        let line: Vec<point::Point>;

        if is_horizontal && !(root.x + length >= self.width && root.x + length < self.width) {
            line = line::line(&root, &point::Point::new(root.x + length, root.y));
        } else if !(root.y + length >= self.height && root.y + length < self.height) {
            line = line::line(&root, &point::Point::new(root.x, root.y + length));
        } else {
            line = line::line(&root, &root);
        }
        return line;
    }

    pub fn generate_segment(&mut self) -> wall::Wall {
        let mut root = self.generate_wall();
        root.child = self.generate_child(&root);

        return root;
    }

    fn generate_child(&self, wall: &wall::Wall) -> Option<Box<wall::Wall>> {
        let child_index = rand::thread_rng().gen_range(-1..wall.length as i64) as i64;
        if child_index < 0 {
            return None;
        } else {
            let child_length = rand::thread_rng().gen_range(1..5) as usize;
            let line = self.draw_line(&wall.points[child_index as usize], !wall.is_horizontal, child_length);

            return Some(Box::new(wall::Wall::new(line, child_length, !wall.is_horizontal, None)));
        }
    }

    pub fn generate_wall(&self) -> wall::Wall {
        let root = point::Point::new(rand::thread_rng().gen_range(0..self.height) as usize,
                                     rand::thread_rng().gen_range(0..self.width) as usize);
        let length = rand::thread_rng().gen_range(1..5) as usize;
        let is_horizontal = rand::thread_rng().gen_range(0..2) != 0;

        let line = self.draw_line(&root, is_horizontal, length);
        
        return wall::Wall::new(line, length, is_horizontal, None);
    }
}


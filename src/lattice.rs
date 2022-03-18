use colored::Colorize;
use rand::Rng;
use crate::line;
use crate::point;

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

    pub fn draw_line(&mut self, p1: &point::Point, p2: &point::Point) {
        let line = line::line(p1, p2);
        for pt in line {
            self.data[pt.y][pt.x] = '#';
        }
    }

    fn generate_segment(&mut self) {
        
    }

    pub fn generate_wall(&self) -> Vec<point::Point> {
        let root = point::Point::new(rand::thread_rng().gen_range(0..self.height) as usize,
                                     rand::thread_rng().gen_range(0..self.width) as usize);
        let length = rand::thread_rng().gen_range(1..5) as usize;
        let is_horizontal = rand::thread_rng().gen_range(0..2) != 0;
        println!("{}", is_horizontal);

        let line: Vec<point::Point>;
        if is_horizontal && !(root.x + length >= self.width ) {
            line = line::line(&root, &point::Point::new(root.x + length, root.y));
        } else if !(root.y + length >= self.height ) {
            line = line::line(&root, &point::Point::new(root.x, root.y + length));
        } else {
            line = line::line(&root, &root);
        }

        return line;
    }
}


mod point;
mod lattice;
mod line;

fn main() {
    
    let mut l1 = lattice::Lattice::new(50, 100);

    //l1.draw_line(&point::Point::new(0, 0), &point::Point::new(20, 90));
    let wall = l1.generate_wall();
    for pt in wall {
        l1.data[pt.y][pt.x] = '#';
    }
    l1.print();
}

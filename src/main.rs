mod point;
mod lattice;
mod line;
mod wall;

fn main() {
    
    let mut l1 = lattice::Lattice::new(25, 50);

    //l1.draw_line(&point::Point::new(0, 0), &point::Point::new(20, 90));
    let wall = l1.generate_segment();
    for pt in wall.points {
        l1.data[pt.y][pt.x] = '#';
    }

    if !wall.child.is_none() {
        for pt in (*(wall.child.unwrap())).points {
            l1.data[pt.y][pt.x] = '#';
        }
    }
    
    l1.print();
}

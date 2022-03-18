mod point;
mod lattice;
mod line;
mod wall;

fn main() {
    
    let mut l1 = lattice::Lattice::new(25, 50);

    let mut wall = l1.generate_segment();
    
    l1.print();
}

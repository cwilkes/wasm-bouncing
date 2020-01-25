
use mylib::Universe;



pub fn main() {
    let mut universe = Universe::new();
    println!("{}", universe);
    universe.advance();
    println!("{}", universe);
}

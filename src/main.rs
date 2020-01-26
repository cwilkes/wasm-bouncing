
use mylib::Universe;



pub fn main() {
    let mut universe = Universe::new();
    for _i in 0..125 {
        println!("{:?}", universe);
        universe = universe.advance();
    }
}

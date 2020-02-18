extern crate libc;
mod goat;
use goat::{*};

fn main() {
    println!("Hello GOAT!");
    rs_goat_init();
    {
        let g = Goat::random();
        let m = g.mesh();
        let (v,f) = m.buffers();
        g.dump();

    }
    rs_goat_exit();
}

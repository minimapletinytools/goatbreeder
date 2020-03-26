extern crate libc;
mod goat;
use goat::{*};

fn main() {
    println!("Hello GOAT!");
    rs_goat_init();
    {
        println!("generating goat");
        let g = Goat::random();

        println!("printing mesh");
        let m = g.mesh();
        let (v,f) = m.buffers();

        // print buffers ourselves
        println!("{:?}", v);
        println!("{:?}", f);

        // print using library
        g.dump();

        println!("breeding");

        // test breeding
        let g1 = Goat::random();
        let g2 = Goat::random();
        let g3 = breed(&g1, &g2);

        // print to check results
        g3.dump();

        println!("done");
    }
    rs_goat_exit();
}

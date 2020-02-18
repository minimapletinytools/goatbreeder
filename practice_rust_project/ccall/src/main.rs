extern crate libc;
mod goat;
use goat::rs_goat_init;
use goat::rs_goat_exit;

fn main() {
    println!("Hello GOAT!");
    rs_goat_init();

    rs_goat_exit();
}

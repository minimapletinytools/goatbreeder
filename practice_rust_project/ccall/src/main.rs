extern crate libc;
use libc::c_void;

//#[link(name = "animalclub")]
extern "C" {
    fn my_init();
    fn my_exit();
    //fn breed(seed: c_int, dna1: *mut char, dna2: *mut char, outdna: *mut char, size: c_int);
    fn dump_goat(goat: *const c_void);
    fn random_goat() -> *const c_void;
    fn free_goat(goat: *const c_void);
    fn breed_goat(goat1: *const c_void, goat2: *const c_void) -> *const c_void;
    fn goat_mesh(goat: *const c_void) -> *mut c_void;
    fn free_goat_mesh(goat: *const c_void);
}

fn main() {
    println!("Hello, world!aonethuonateh");
    unsafe {
        my_init();
        let g = random_goat();
        dump_goat(g);
        free_goat(g);
        my_exit();
    }
}

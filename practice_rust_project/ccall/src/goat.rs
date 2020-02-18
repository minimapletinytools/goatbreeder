extern crate libc;
use libc::c_void;
use std::slice::from_raw_parts;

//#[link(name = "animalclub")]
extern "C" {
    fn my_init();
    fn my_exit();
    //fn breed(seed: c_int, dna1: *mut char, dna2: *mut char, outdna: *mut char, size: c_int); // commented out to exceed 80 chars
    fn dump_goat(goat: *const c_void);
    fn random_goat() -> *const c_void;
    fn free_goat(goat: *const c_void);
    fn breed_goat(goat1: *const c_void, goat2: *const c_void) -> *const c_void;
    fn goat_mesh(goat: *const c_void) -> *const MeshInternal;
    fn free_goat_mesh(goat: *const MeshInternal);
}

pub fn rs_goat_init() {
    unsafe { my_init(); }
}

pub fn rs_goat_exit() {
    unsafe { my_exit(); }
}

#[repr(C)]
struct MeshInternal {
    vertices: *const c_void
    , vertex_count: u32
    , faces: *const c_void
    , face_count: u32
}

struct Mesh<'a> {
    mesh: &'a const MeshInternal
}

impl<'a> Mesh<'a> {
    //(&a' [f32], &a' [u32]) {
    pub fn mesh(&self) -> &'a [f32] {
        let r1 = unsafe { from_raw_parts(self.mesh.vertices, self.vertex_count) };
        let r2 = unsafe { from_raw_parts(self.mesh.vertices, self.vertex_count) };
        r1
    }
}
impl<'a> Drop for Mesh<'a> {
    fn drop(&mut self) {
        unsafe { free_goat_mesh(self.mesh); }
    }
}

pub struct Goat {
    hsptr: *const c_void
}

impl Goat {
    pub fn random() -> Self {
        let hsptr: *const c_void = unsafe { random_goat() };
        Goat { hsptr }
    }
    pub fn mesh<'a>(self: Goat) -> Mesh<'a> {
        let mesh = unsafe { goat_mesh(self.hsptr) };
        Mesh { mesh }
    }
    pub fn dump(self: Goat) {
        unsafe { dump_goat(self.hsptr); }
    }
}

impl Drop for Goat {
    fn drop(&mut self) {
        unsafe { free_goat(self.hsptr); }
    }
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

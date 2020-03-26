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

pub struct Mesh<'a> {
    mesh: &'a MeshInternal
}

impl<'a> Mesh<'a> {
    pub fn buffers(&self) -> (&'a [f32], &'a [u32]) {
        let r1 = unsafe { from_raw_parts(self.mesh.vertices as *const f32, self.mesh.vertex_count as usize) };
        let r2 = unsafe { from_raw_parts(self.mesh.faces as *const u32, self.mesh.face_count as usize) };
        return (r1, r2);
    }
}
impl<'a> Drop for Mesh<'a> {
    fn drop(&mut self) {
        let raw = self.mesh as *const MeshInternal;
        unsafe { free_goat_mesh(raw); }
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
    pub fn mesh<'a>(&self) -> Mesh<'a> {
        let mesh = unsafe { &*goat_mesh(self.hsptr) };
        Mesh { mesh: &mesh }
    }
    pub fn dump(&self) {
        unsafe { dump_goat(self.hsptr); }
    }
}

impl Drop for Goat {
    fn drop(&mut self) {
        unsafe { free_goat(self.hsptr); }
    }
}

pub fn breed(g1: &Goat, g2: &Goat) -> Goat {
    let hsptr: *const c_void = unsafe { breed_goat(g1.hsptr, g2.hsptr) };
    Goat { hsptr }
}

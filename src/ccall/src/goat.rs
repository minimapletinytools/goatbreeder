extern crate libc;
use libc::c_void;
use std::slice::from_raw_parts;

// for writing obj file
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::Path;

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
    unsafe {
        my_init();
    }
}

pub fn rs_goat_exit() {
    unsafe {
        my_exit();
    }
}

#[repr(C)]
struct MeshInternal {
    vertices: *const c_void,
    vertices_count: u32,
    normals: *const c_void,
    normals_count: u32,
    uvs: *const c_void,
    uvs_count: u32,
    faces: *const c_void,
    faces_count: u32,
}

pub struct Mesh<'a> {
    mesh: &'a MeshInternal,
}

impl<'a> Mesh<'a> {
    pub fn buffers(&self) -> (&'a [f32], &'a [f32], &'a [f32], &'a [u32]) {
        let p = unsafe {
            from_raw_parts(
                self.mesh.vertices as *const f32,
                self.mesh.vertices_count as usize,
            )
        };
        let n = unsafe {
            from_raw_parts(
                self.mesh.normals as *const f32,
                self.mesh.normals_count as usize,
            )
        };
        let tc =
            unsafe { from_raw_parts(self.mesh.uvs as *const f32, self.mesh.uvs_count as usize) };
        let f = unsafe {
            from_raw_parts(
                self.mesh.faces as *const u32,
                self.mesh.faces_count as usize,
            )
        };
        (p, n, tc, f)
    }
}

// for testing
pub fn write_obj_from_buffers(p: &[f32], n: &[f32], tc: &[f32], f: &[u32]) -> std::io::Result<()> {
    let path = Path::new("goat.obj");
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut file = LineWriter::new(file);

    file.write_all(b"#beginning of mesh obj file \ng\n")?;

    for x in p.to_vec().chunks(3) {
        file.write_all(format!("v {} {} {}\n", x[0], x[1], x[2]).as_bytes())?;
    }

    for x in n.to_vec().chunks(3) {
        file.write_all(format!("vn {} {} {}\n", x[0], x[1], x[2]).as_bytes())?;
    }

    for x in tc.to_vec().chunks(2) {
        file.write_all(format!("vt {} {}\n", x[0], x[1]).as_bytes())?;
    }

    for x in f.to_vec().chunks(3) {
        file.write_all(
            format!(
                "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                x[0] + 1,
                x[0] + 1,
                x[0] + 1,
                x[1] + 1,
                x[1] + 1,
                x[1] + 1,
                x[2] + 1,
                x[2] + 1,
                x[2] + 1
            )
            .as_bytes(),
        )?;
    }

    file.flush()?;
    Ok(())
}

impl<'a> Drop for Mesh<'a> {
    fn drop(&mut self) {
        let raw = self.mesh as *const MeshInternal;
        unsafe {
            free_goat_mesh(raw);
        }
    }
}

pub struct Goat {
    hsptr: *const c_void,
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
        unsafe {
            dump_goat(self.hsptr);
        }
    }
}

impl Drop for Goat {
    fn drop(&mut self) {
        unsafe {
            free_goat(self.hsptr);
        }
    }
}

pub fn breed(g1: &Goat, g2: &Goat) -> Goat {
    let hsptr: *const c_void = unsafe { breed_goat(g1.hsptr, g2.hsptr) };
    Goat { hsptr }
}

// add the folowing to your ~/.cargo/config
// for rpath to get set properly on Mac
//[target.x86_64-apple-darwin]
//rustflags = ["-C", "link-arg=-Wl,-rpath,../../,-rpath,./"]

fn main() {
    println!("cargo:rustc-link-lib=dylib=animalclub");
    println!("cargo:rustc-link-search=native=./");
}

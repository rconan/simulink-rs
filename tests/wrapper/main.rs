use simulink_rs::Sys;
use std::env;
use std::path::Path;

#[test]
fn main() {
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tests")
        .join("wrapper");
    let sys = Sys::builder().folder(path.to_str().unwrap()).build();
    dbg!(&sys);
    println!("{sys}");
}

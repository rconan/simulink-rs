# Template for GMT Simulink C controller 

The template directory is the blueprint for importing generated C code from Simulink into Rust.

Simply copy and rename the `template` directory, then:

 * in `Cargo.toml`, set the package name,
 * copy all the Simulink source (.c) and header (.h) files into the `sys` folder,
 * in `build.rs`, pass the name of the Rust structure that will implement the Rust version of the Simulink controller as argument to `simulink_rs::Sys::new(Some("..."))`,

and finally run `cargo build` to check that the new crate is build without errors.

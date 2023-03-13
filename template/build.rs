fn main() {
    let sys = simulink_rs::Sys::new(Some("..."));
    sys.compile().generate_module();
}

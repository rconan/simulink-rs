//! # Simulink C Rust wrapper and binder
//!
//! A Rust library to import generated C code from Simulink in Rust
//!
//! # Example
//! ```ignore
//! let sys = Sys::new(Some("MySimulinkController"));
//! sys.compile().generate_module();
//! ```

use regex::Regex;
use std::{
    env,
    fmt::Display,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

/// Simulink inputs/outputs/states
#[derive(Debug, Default)]
struct IO {
    /// i/o variable name
    pub name: String,
    /// i/o variable size
    pub size: Option<usize>,
}
impl IO {
    /// Creates a new IO
    fn new(name: &str, size: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            size: size.and_then(|s| s.parse().ok()),
        }
    }
}
/// List of Simulink inputs/outputs/states
#[derive(Debug, Default)]
struct List(Vec<IO>);
impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var: Vec<_> = self
            .0
            .iter()
            .map(|IO { name, size }| {
                if let Some(size) = size {
                    format!("{}: [0f64; {}]", name, size)
                } else {
                    format!("{}: 0f64", name)
                }
            })
            .collect();
        write!(f, "{}", var.join(","))
    }
}

/// Simulink model description
#[derive(Debug, Default)]
struct Model {
    name: String,
    inputs: List,
    outputs: List,
    states: List,
}
impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r"
        /// Simulink controller wrapper
        #[derive(Debug, Clone, Copy, Default)]
        pub struct {model} {{
            // Inputs Simulink structure
            pub inputs: ExtU_{model}_T,
            // Outputs Simulink structure
            pub outputs: ExtY_{model}_T,
            states: DW_{model}_T,
        }}
        impl Default for ExtU_{model}_T {{
            fn default() -> Self {{
                Self {{ {var_u} }}
            }}
        }}
        impl Default for ExtY_{model}_T {{
            fn default() -> Self {{
                Self {{ {var_y} }}
            }}
        }}
        impl Default for DW_{model}_T {{
            fn default() -> Self {{
                Self {{ {var_x} }}
            }}
        }}
        impl {model} {{
            /// Creates a new controller
            pub fn new() -> Self {{
                let mut this: Self = Default::default();
                let mut data: RT_MODEL_{model}_T = tag_RTM_{model}_T {{
                    dwork: &mut this.states as *mut _,
                }};
                unsafe {{
                     {model}_initialize(&mut data as *mut _)
                }}
                this
            }}
            /// Steps the controller
            pub fn step(&mut self) {{
                let mut data: RT_MODEL_{model}_T = tag_RTM_{model}_T {{
                    dwork: &mut self.states as *mut _,
                }};
                unsafe {{
                    {model}_step(
                        &mut data as *mut _,
                        &mut self.inputs as *mut _,
                        &mut self.outputs as *mut _,
                    )
                }}
            }}
        }}        
        ",
            model = self.name,
            var_u = self.inputs.to_string(),
            var_y = self.outputs.to_string(),
            var_x = self.states.to_string(),
        )
    }
}

/// Parse the Simulink C header file to extract inputs, outputs or states variables
fn parse_io(lines: &mut std::io::Lines<BufReader<File>>, io: &str) -> Option<List> {
    let re = Regex::new(r"_T (?P<name>\w+)(?:\[(?P<size>\d+)\])?").unwrap();
    match lines.next() {
        Some(Ok(line)) if line.starts_with("typedef struct") => {
            println!("| {}:", io);
            let mut io_data = vec![];
            while let Some(Ok(line)) = lines.next() {
                if line.contains(io) {
                    break;
                } else {
                    if let Some(caps) = re.captures(&line) {
                        let size = caps.name("size").map(|m| m.as_str());
                        println!("|  - {:<22}: {:>5}", &caps["name"], size.unwrap_or("1"),);
                        io_data.push(IO::new(&caps["name"], size))
                    }
                }
            }
            Some(List(io_data))
        }
        _ => None,
    }
}

/// Simulink control system C source and header files parser and builder
///
/// # Example
/// ```ignore
/// let sys = Sys::new(Some("MySimulinkController"));
/// sys.compile().generate_module();
/// ```
pub struct Sys {
    controller: Option<String>,
    sources: Vec<PathBuf>,
    headers: Vec<PathBuf>,
}

impl Sys {
    /// Create a new Simulink FFI
    ///
    /// The Simulink controlller will be given the type `rs_type` if present
    pub fn new<S: Into<String>>(rs_type: Option<S>) -> Self {
        let sys = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("sys");

        let mut sources = vec![];
        let mut headers = vec![];

        if let Ok(entries) = fs::read_dir(&sys) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.path();
                    if let Some(extension) = file_name.extension() {
                        match extension.to_str() {
                            Some("c") => {
                                sources.push(file_name);
                            }
                            Some("h") => {
                                headers.push(file_name);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        Self {
            controller: rs_type.map(|x| x.into()),
            sources,
            headers,
        }
    }
    /// Returns the main header file
    fn header(&self) -> Option<&str> {
        self.headers.iter().find_map(|header| {
            header.to_str().filter(|f| {
                !(f.ends_with("rtwtypes.h")
                    || f.ends_with("rt_defines.h")
                    || f.ends_with("_private.h")
                    || f.ends_with("_types.h"))
            })
        })
    }
    /// Parses the main header file into [Model]
    ///
    /// Extract the model name and the lists of inputs, outputs and states variables
    /// and creates a [Model]
    fn parse_header(&self) -> Model {
        let Some(header) = self.header() else { panic!("cannot find error in sys")};
        let file = File::open(header).expect(&format!("file {:?} not found", header));
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut model = Model::default();
        model.name = loop {
            if let Some(Ok(line)) = lines.next() {
                if line.contains("File:") {
                    let regex = Regex::new(r"File:\s*(\w+)\.h").unwrap();
                    if let Some(captures) = regex.captures(&line) {
                        let name = captures.get(1).unwrap().as_str();
                        break name.to_string();
                    }
                }
            }
        };
        while let Some(Ok(line)) = lines.next() {
            if line.contains("External inputs") {
                model.inputs = parse_io(&mut lines, "ExtU").unwrap();
            }
            if line.contains("External outputs") {
                model.outputs = parse_io(&mut lines, "ExtY").unwrap();
            }
            if line.contains("Block signals") {
                model.states = parse_io(&mut lines, "DW").unwrap();
            }
        }
        model
    }
    /// Compiles the Simulink C model
    pub fn compile(&self) -> &Self {
        let mut cc_builder = cc::Build::new();
        self.sources
            .iter()
            .fold(&mut cc_builder, |cc_builder, source| {
                cc_builder.file(source)
            });
        let bindings_builder = self
            .headers
            .iter()
            .fold(bindgen::builder(), |bindings, header| {
                println!("cargo:rerun-if-changed={:}", header.to_str().unwrap());
                bindings.header(
                    header
                        .to_str()
                        .expect(&format!("{:?} conversion to str failed", header)),
                )
            });

        let lib = env::var("CARGO_PKG_NAME").unwrap();
        println!("cargo:rustc-link-search=native=lib{}", lib);
        println!("cargo:rustc-link-lib={}", lib);

        cc_builder.compile(lib.as_str());
        let bindings = bindings_builder
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");
        let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
        self
    }
    /// Generates the controller.rs module
    pub fn generate_module(&self) {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("controller.rs");
        fs::write(&dest_path, format!("{}", self)).unwrap();
    }
}

impl Display for Sys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model = self.parse_header();
        if let Some(controller) = self.controller.as_ref() {
            writeln!(f, "/// Rust binder to Simulink C controller wrapper")?;
            writeln!(f, "#[allow(dead_code)]")?;
            writeln!(f, "pub type {} = {};", controller, model.name)?;
        }
        model.fmt(f)
    }
}
/*
pub fn build() {
    let sys = Sys::new();
    sys.compile().generate_module();
} */

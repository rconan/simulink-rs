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
    fmt::{Debug, Display},
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

mod model;
use model::{Model, Simulink};

/// Simulink control system C source and header files parser and builder
///
/// # Example
/// ```ignore
/// let sys = Sys::new(Some("MySimulinkController"));
/// sys.compile().generate_module();
/// ```
#[derive(Debug, Default, Clone)]
pub struct Sys {
    controller: Option<String>,
    sources: Vec<PathBuf>,
    headers: Vec<PathBuf>,
}

pub struct Builder {
    controller_type: Option<String>,
    sys_folder: String,
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            controller_type: Default::default(),
            sys_folder: "sys".into(),
        }
    }
}
impl Builder {
    /// Sets the name of the Rust structure that acts as a wrapper for the Simulink C code
    ///
    /// If not set, the structure is given the same name than the Simulink control model
    pub fn name<S: Into<String>>(mut self, rs_type: S) -> Self {
        self.controller_type = Some(rs_type.into());
        self
    }
    /// Sets the name of the folder with the C header and source file
    ///
    /// If not set, expect the folder to be named "sys"
    pub fn folder<S: Into<String>>(mut self, folder: S) -> Self {
        self.sys_folder = folder.into();
        self
    }
    /// Builds a new Simulink C to Rust wrapper
    pub fn build(self) -> Sys {
        let sys = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(self.sys_folder);

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

        Sys {
            controller: self.controller_type,
            sources,
            headers,
        }
    }
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
    /// Creates a builder for the Simulink C to Rust wrapper
    pub fn builder() -> Builder {
        Default::default()
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
        let Some(header) = self.header() else {
            panic!("cannot find error in sys")
        };
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
        while let Some(data) = Simulink::parse_io(&mut lines) {
            model.simulink.push(data);
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
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
        <Model as Display>::fmt(&model, f)
    }
}

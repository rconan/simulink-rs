use regex::Regex;
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::BufReader,
};

mod simulink;
pub use simulink::{Simulink, IO};

/// Simulink model description
#[derive(Default, Debug)]
pub struct Model {
    pub name: String,
    pub simulink: Vec<Simulink>,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r"
/// Simulink controller wrapper
#[derive(Clone, Copy, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct {model} {{
    // Inputs Simulink structure
    pub inputs: ExtU_{model}_T,
    // Outputs Simulink structure
    pub outputs: ExtY_{model}_T,
    states: DW_{model}_T,
}}",
            model = self.name,
        )?;

        for simulink in &self.simulink {
            writeln!(
                f,
                r"
{}",
                simulink.default_as_string()
            )?;
        }
        writeln!(
            f,
            r"
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
{serde}
        ",
            model = self.name,
            serde = self
                .simulink
                .iter()
                .map(|simulink| simulink.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Simulink {
    /// Parse the Simulink C header file to extract inputs, outputs or states variables
    pub fn parse_io(lines: &mut std::io::Lines<BufReader<File>>) -> Option<Self> {
        let re_prop = Regex::new(r"(?P<dtype>\w+)\s(?P<name>\w+)(?:\[(?P<size>\d+)\])?").unwrap();
        let re_struct = Regex::new(r"} (\w+);").unwrap();
        let mut this: Option<Self> = Default::default();
        'header: loop {
            match lines.next() {
                Some(Ok(line)) if line.starts_with("typedef struct") => {
                    println!("| Struct:");
                    while let Some(Ok(line)) = lines.next() {
                        if let Some(caps) = re_struct.captures(&line) {
                            let name = caps.get(1).unwrap().as_str().to_string();
                            if name.starts_with("ConstP") {
                                this = None;
                                continue 'header;
                            }
                            println!("| {}", name);
                            this.get_or_insert(Default::default()).name = name;
                            break 'header;
                        }
                        if let Some(caps) = re_prop.captures(&line) {
                            let size = caps.name("size").map(|m| m.as_str());
                            println!("|  - {:<22}: {:>5}", &caps["name"], size.unwrap_or("1"),);
                            this.get_or_insert(Default::default())
                                .properties
                                .push(IO::new(&caps["dtype"], &caps["name"], size));
                        }
                    }
                }
                Some(_) => continue 'header,
                None => break 'header,
            }
        }
        this
    }
    fn default_as_string(&self) -> String {
        format!(
            r"
impl Default for {name} {{
    fn default() -> Self {{
        Self {{ {properties} }}
    }}
}}
        ",
            name = self.name,
            properties = self.properties.to_string()
        )
    }
}

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

/// Simulink structure properties
#[derive(Debug, Default)]
pub struct IO {
    /// i/o variable name
    pub dtype: String,
    /// i/o variable name
    pub name: String,
    /// i/o variable size
    pub size: Option<usize>,
}
impl IO {
    /// Creates a new property
    pub fn new(dtype: &str, name: &str, size: Option<&str>) -> Self {
        Self {
            dtype: dtype.to_string(),
            name: name.to_string(),
            size: size.and_then(|s| s.parse().ok()),
        }
    }
}
/// List of Simulink properties
#[derive(Debug, Default)]
pub struct List(Vec<IO>);
impl Deref for List {
    type Target = Vec<IO>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var: Vec<_> = self
            .0
            .iter()
            .map(|IO { name, size, .. }| {
                if let Some(size) = size {
                    format!("{}: [Default::default(); {}]", name, size)
                } else {
                    format!("{}: Default::default()", name)
                }
            })
            .collect();
        writeln!(f, "\n{}", var.join(",\n"))
    }
}

/// Simulink structure
#[derive(Debug, Default)]
pub struct Simulink {
    pub name: String,
    pub properties: List,
}
pub trait SerializeImpl {
    fn serialize_impl(&self) -> String;
}
pub trait DeserializeImpl {
    fn deserialize_impl(&self) -> String;
}

mod deserialize;
mod serialize;

impl Display for Simulink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.serialize_impl())?;
        writeln!(f, "{}", self.deserialize_impl())
    }
}

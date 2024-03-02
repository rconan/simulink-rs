use super::{List, SerializeImpl, Simulink, IO};

impl SerializeImpl for IO {
    fn serialize_impl(&self) -> String {
        if self.size.is_some() {
            format!(
                r#"
        ::serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "{field}",
            &Vec::from(&self.{field}),
        )?;
            "#,
                field = self.name
            )
        } else {
            format!(
                r#"
        ::serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "{field}",
            &self.{field},
        )?;
            "#,
                field = self.name
            )
        }
    }
}

impl SerializeImpl for List {
    fn serialize_impl(&self) -> String {
        self.iter()
            .map(|io| io.serialize_impl())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl SerializeImpl for Simulink {
    fn serialize_impl(&self) -> String {
        format!(
            r#"    
impl ::serde::ser::Serialize for {sim} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {{
        let mut serde_state = serializer.serialize_struct("{sim}", {n_field})?;
        {fields}
        ::serde::ser::SerializeStruct::end(serde_state)
    }}
}}
                        "#,
            sim = self.name,
            fields = self.properties.serialize_impl(),
            n_field = self.properties.len(),
        )
    }
}

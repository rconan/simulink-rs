use super::{DeserializeImpl, List, Simulink, IO};

impl DeserializeImpl for IO {
    fn deserialize_impl(&self) -> String {
        format!(r#""{0}" => Ok(Field::{0})"#, self.name)
    }
}

impl DeserializeImpl for List {
    fn deserialize_impl(&self) -> String {
        self.iter()
            .map(|field| field.deserialize_impl())
            .collect::<Vec<String>>()
            .join(",\n")
    }
}

pub trait Visitor {
    fn visit(&self) -> String;
}
impl Visitor for List {
    fn visit(&self) -> String {
        let ((v1, v2), v3): ((Vec<_>, Vec<_>), Vec<_>) = self
            .iter()
            .map(|field| {
                let a = format!("let mut {0} = None;", field.name);
                let b = format!(
                    r#"
    Field::{0} => {{
        if {0}.is_some() {{
            return Err(::serde::de::Error::duplicate_field("{0}"));
        }}
        {0} = Some(map.next_value::<{1}>()?);
    }}
            "#,
                    field.name,
                    if field.size.is_none() {
                        field.dtype.clone()
                    } else {
                        format!("Vec<{}>", field.dtype)
                    }
                );
                let c = if let Some(size) = field.size {
                    format!(
                        r#"
    let {0}: [f64; {1}] = {0}
    .ok_or_else(|| ::serde::de::Error::missing_field("{0}"))?
    .try_into()
    .map_err(|_| {{
        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Seq, &self)
    }})?;
                    "#,
                        field.name, size
                    )
                } else {
                    format!(
                        r#"
    let {0} = {0}.ok_or_else(|| ::serde::de::Error::missing_field("{0}"))?;                    
                "#,
                        field.name
                    )
                };
                ((a, b), c)
            })
            .unzip();
        format!(
            r#"
{0}
while let Some(key) = map.next_key::<Field>()? {{
    match key {{
{1}
    }}
}}
{2}
            "#,
            v1.join("\n"),
            v2.join("\n"),
            v3.join("\n"),
        )
    }
}

impl DeserializeImpl for Simulink {
    fn deserialize_impl(&self) -> String {
        let fields: Vec<_> = self
            .properties
            .iter()
            .map(|prop| format!("{}", prop.name))
            .collect();
        format!(
            r#"
impl<'de> ::serde::de::Deserialize<'de> for {sim} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {{
        const FIELDS: &'static [&'static str] = &[{fields_str}];

        enum Field {{
{fields}
        }}

        impl<'de> ::serde::de::Deserialize<'de> for Field {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {{
                struct FieldVisitor;
                impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {{
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {{
                        formatter.write_str("field identifier")
                    }}

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {{
                        match value {{
{fields_match},
_ => Err(::serde::de::Error::unknown_field(value, FIELDS)),
                        }}
                    }}
                }}
                deserializer.deserialize_identifier(FieldVisitor)
            }}
        }}

        struct SimulinkVisitor;
        impl<'de> ::serde::de::Visitor<'de> for SimulinkVisitor {{
            type Value = {sim};
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {{
                formatter.write_str("struct {sim}")
            }}
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {{
{visitor}
Ok({sim} {{
    {fields}
}})
            }}
        }}
        deserializer.deserialize_struct("{sim}", FIELDS, SimulinkVisitor)
    }}
}}
                "#,
            sim = self.name,
            fields = fields.join(",\n"),
            fields_str = fields
                .iter()
                .map(|field| format!(r#""{field}""#))
                .collect::<Vec<String>>()
                .join(", "),
            fields_match = self.properties.deserialize_impl(),
            visitor = self.properties.visit(),
        )
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/* pub struct ExtU_M1SA_Control_CS_T {
    pub LC_FxyzMxyz_CG: [real_T; 6],
    pub SA_offsetF_cmd: [real_T; 306],
} */

#[derive(Debug, PartialEq)]
pub struct ExtU_M1SA_Control_CS_T {
    pub LC_FxyzMxyz_CG: [f64; 2],
    pub SA_offsetF_cmd: [f64; 306],
    pub scalar: f64,
}
impl ::serde::ser::Serialize for ExtU_M1SA_Control_CS_T {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        let mut serde_state = serializer.serialize_struct("ExtU_M1SA_Control_CS_T", 2)?;
        ::serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "LC_FxyzMxyz_CG",
            &Vec::from(&self.LC_FxyzMxyz_CG),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "SA_offsetF_cmd",
            &Vec::from(&self.SA_offsetF_cmd),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut serde_state, "scalar", &self.scalar)?;
        ::serde::ser::SerializeStruct::end(serde_state)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for ExtU_M1SA_Control_CS_T {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["LC_FxyzMxyz_CG", "SA_offsetF_cmd", "scalar"];

        enum Field {
            LC_FxyzMxyz_CG,
            SA_offsetF_cmd,
            Scalar,
        }

        impl<'de> ::serde::de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("field identifier")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match value {
                            "LC_FxyzMxyz_CG" => Ok(Field::LC_FxyzMxyz_CG),
                            "SA_offsetF_cmd" => Ok(Field::SA_offsetF_cmd),
                            "scalar" => Ok(Field::Scalar),
                            _ => Err(::serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SimulinkVisitor;
        impl<'de> ::serde::de::Visitor<'de> for SimulinkVisitor {
            type Value = ExtU_M1SA_Control_CS_T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct ExtU_M1SA_Control_CS_T")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: ::serde::de::SeqAccess<'de>,
            {
                let LC_FxyzMxyz_CG: [f64; 2] = seq
                    .next_element::<Vec<f64>>()?
                    .ok_or_else(|| ::serde::de::Error::invalid_length(2, &self))?
                    .try_into()
                    .map_err(|_| {
                        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Seq, &self)
                    })?;
                let SA_offsetF_cmd: [f64; 306] = seq
                    .next_element::<Vec<f64>>()?
                    .ok_or_else(|| ::serde::de::Error::invalid_length(306, &self))?
                    .try_into()
                    .map_err(|_| {
                        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Seq, &self)
                    })?;
                let scalar = seq
                    .next_element::<f64>()?
                    .ok_or_else(|| ::serde::de::Error::invalid_length(1, &self))?;
                Ok(ExtU_M1SA_Control_CS_T {
                    LC_FxyzMxyz_CG,
                    SA_offsetF_cmd,
                    scalar,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                let mut LC_FxyzMxyz_CG = None;
                let mut SA_offsetF_cmd = None;
                let mut scalar = None;
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::LC_FxyzMxyz_CG => {
                            if LC_FxyzMxyz_CG.is_some() {
                                return Err(::serde::de::Error::duplicate_field("LC_FxyzMxyz_CG"));
                            }
                            LC_FxyzMxyz_CG = Some(map.next_value::<Vec<f64>>()?);
                        }
                        Field::SA_offsetF_cmd => {
                            if SA_offsetF_cmd.is_some() {
                                return Err(::serde::de::Error::duplicate_field("SA_offsetF_cmd"));
                            }
                            SA_offsetF_cmd = Some(map.next_value::<Vec<f64>>()?);
                        }
                        Field::Scalar => {
                            if scalar.is_some() {
                                return Err(::serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value::<f64>()?);
                        }
                    }
                }
                let LC_FxyzMxyz_CG: [f64; 2] = LC_FxyzMxyz_CG
                    .ok_or_else(|| ::serde::de::Error::missing_field("LC_FxyzMxyz_CG"))?
                    .try_into()
                    .map_err(|_| {
                        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Seq, &self)
                    })?;
                let SA_offsetF_cmd: [f64; 306] = SA_offsetF_cmd
                    .ok_or_else(|| ::serde::de::Error::missing_field("SA_offsetF_cmd"))?
                    .try_into()
                    .map_err(|_| {
                        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Seq, &self)
                    })?;
                let scalar = scalar.ok_or_else(|| ::serde::de::Error::missing_field("scalar"))?;
                Ok(ExtU_M1SA_Control_CS_T {
                    LC_FxyzMxyz_CG,
                    SA_offsetF_cmd,
                    scalar,
                })
            }
        }
        deserializer.deserialize_struct("ExtU_M1SA_Control_CS_T", FIELDS, SimulinkVisitor)
    }
}

#[test]
fn main() {
    let data = ExtU_M1SA_Control_CS_T {
        LC_FxyzMxyz_CG: [1.0, 2.0],
        SA_offsetF_cmd: [0.; 306],
        scalar: 9.87456,
    };
    let serialized = serde_json::to_string(&data).unwrap();
    println!("{}", serialized);
    let deserialized: ExtU_M1SA_Control_CS_T = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
    assert_eq!(data, deserialized);
}

use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

pub mod filter;
pub mod sort;

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub fn deserialize_opt_int_to_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MagVisitor;

    impl<'de> Visitor<'de> for MagVisitor {
        type Value = Option<String>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("'mag' as a number or string")
        }

        fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(id.to_string()))
        }

        fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(id.to_owned()))
        }
    }

    deserializer.deserialize_any(MagVisitor)
}

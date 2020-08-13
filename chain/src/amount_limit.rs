use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::{NumberBytes, Read, SerializeData, Write};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, NumberBytes, Read, Write, SerializeData)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[iost_root_path = "crate"]
pub struct AmountLimit {
    /// token name
    pub token: String,
    /// corresponding token limit
    pub value: String,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for AmountLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Debug)]
        struct VisitorAmountLimit;
        impl<'de> serde::de::Visitor<'de> for VisitorAmountLimit {
            type Value = AmountLimit;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::MapAccess<'de>,
            {
                let mut token = String::from("");
                let mut value = String::from("");
                while let Some(field) = map.next_key()? {
                    match field {
                        "token" => {
                            token = map.next_value()?;
                        }
                        "value" => {
                            value = map.next_value()?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let amount_limit = AmountLimit { token, value };
                Ok(amount_limit)
            }
        }
        deserializer.deserialize_any(VisitorAmountLimit)
    }
}

impl AmountLimit {
    pub fn new(token: String, value: String) -> Self {
        AmountLimit { token, value }
    }
}

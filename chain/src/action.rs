use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

use crate::Error::JsonParserError;
use crate::{AccountName, Error, NumberBytes, Read, ReadError, SerializeData, Write, WriteError};
use codec::{Decode, Encode};
use core::str::FromStr;
#[cfg(feature = "std")]
use serde::{
    ser::{Error as SerError, SerializeStruct, Serializer},
    Deserialize, Serialize,
};
#[cfg(feature = "std")]
use serde_json::to_string as json_to_string;

#[derive(
    Clone, Default, Debug, PartialEq, Read, Write, Encode, Decode, NumberBytes, SerializeData,
)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[iost_root_path = "crate"]
pub struct Action {
    /// contract name
    pub contract: Vec<u8>,
    /// function name of the contract
    pub action_name: Vec<u8>,
    /// Specific parameters of the call. Put every parameter in an array, and JSON-serialize this array. It may looks like ["a_string", 13]
    pub data: Vec<u8>,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Debug)]
        struct VisitorAction;
        impl<'de> serde::de::Visitor<'de> for VisitorAction {
            type Value = Action;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::MapAccess<'de>,
            {
                let mut contract = String::from("");
                let mut action_name = String::from("");
                let mut data = String::from("");
                while let Some(field) = map.next_key()? {
                    match field {
                        "contract" => {
                            contract = map.next_value()?;
                        }
                        "action_name" => {
                            action_name = map.next_value()?;
                        }
                        "data" => {
                            data = map.next_value()?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let action = Action {
                    contract: contract.into_bytes(),
                    action_name: action_name.into_bytes(),
                    data: data.into_bytes(),
                };
                Ok(action)
            }
        }
        deserializer.deserialize_any(VisitorAction)
    }
}

impl Action {
    pub fn new(contract: String, action_name: String, data: String) -> Self {
        Action {
            contract: contract.into_bytes(),
            action_name: action_name.into_bytes(),
            data: data.into_bytes(),
        }
    }

    // #[cfg(feature = "std")]
    pub fn from_str<T: AsRef<str>>(
        contract: T,
        action_name: T,
        action_transfer: ActionTransfer,
    ) -> crate::Result<Self> {
        // let data = serde_json::to_string(&action_transfer).unwrap();
        Ok(Action {
            contract: contract.as_ref().as_bytes().to_vec(),
            action_name: action_name.as_ref().as_bytes().to_vec(),
            data: "".as_bytes().to_vec(),
        })
    }

    // #[cfg(feature = "std")]
    pub fn transfer<T: AsRef<str>>(from: T, to: T, quantity: T, memo: T) -> crate::Result<Action> {
        let action_transfer = ActionTransfer::from_str(from, to, quantity, memo)?;
        Action::from_str("token.iost", "transfer", action_transfer)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        // result.write(self.contract.as_bytes());
        // result.write(self.action_name.as_bytes());
        // result.write(self.data.as_bytes());
        result
    }
}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "contract: {}\n\
            action_name: {}\n\
            data: {}",
            String::from_utf8_lossy(self.contract.as_slice()),
            String::from_utf8_lossy(self.action_name.as_slice()),
            String::from_utf8_lossy(self.data.as_slice()),
        )
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Read, Write, NumberBytes, Default, SerializeData)]
#[iost_root_path = "crate"]
pub struct ActionTransfer {
    pub token_type: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub memo: String,
}

impl ActionTransfer {
    pub fn new(token_type: String, from: String, to: String, amount: String, memo: String) -> Self {
        ActionTransfer {
            token_type,
            from,
            to,
            amount,
            memo,
        }
    }

    pub fn from_str<T: AsRef<str>>(from: T, to: T, amount: T, memo: T) -> crate::Result<Self> {
        Ok(ActionTransfer {
            token_type: String::from("iost"),
            from: from.as_ref().to_string(),
            to: to.as_ref().to_string(),
            amount: amount.as_ref().to_string(),
            memo: memo.as_ref().to_string(),
        })
    }
}

pub trait ToAction: Write + NumberBytes {
    const NAME: u64;

    #[inline]
    fn to_action(
        &self,
        contract: String,
        action_name: String,
        data: String,
    ) -> core::result::Result<Action, Error> {
        // let mut data = vec![0_u8; self.num_bytes()];
        // self.write(&mut data, &mut 0).unwrap();

        Ok(Action {
            contract: contract.into_bytes(),
            action_name: action_name.into_bytes(),
            data: data.into_bytes(),
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            contract: s.to_string().into_bytes(),
            action_name: s.to_string().into_bytes(),
            data: s.to_string().into_bytes(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action() {
        let action = Action {
            contract: "iost".to_string().into_bytes(),
            action_name: "iost".to_string().into_bytes(),
            data: "".to_string().into_bytes(),
        };
        // let bytes = "action".to_string().into_bytes();
        // dbg!(bytes.clone());
        // dbg!(String::from_utf8_lossy(bytes.as_slice()));
    }

    #[test]
    fn test_action_deserialize_should_be_ok1() {
        let action_str = r#"
        {
            "contract": "token.iost",
            "action_name": "transfer",
            "data": "["iost", "testaccount", "anothertest", "100", "this is an example transfer"]"
        }
        "#;
        let result_action: Result<Action, _> = serde_json::from_str(action_str);
        dbg!(result_action);
        // assert!(result_action.is_err());
    }
}

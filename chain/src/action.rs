use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

use crate::Error::JsonParserError;
use crate::{AccountName, Error, NumberBytes, Read, ReadError, SerializeData, Write, WriteError};
use core::str::FromStr;
#[cfg(feature = "std")]
use serde::{
    ser::{Error as SerError, SerializeStruct, Serializer},
    Deserialize, Serialize,
};
#[cfg(feature = "std")]
use serde_json::to_string as json_to_string;

#[derive(Clone, Default, Debug, PartialEq, Read, Write, NumberBytes, SerializeData)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[iost_root_path = "crate"]
pub struct Action {
    /// contract name
    pub contract: String,
    /// function name of the contract
    pub action_name: String,
    /// Specific parameters of the call. Put every parameter in an array, and JSON-serialize this array. It may looks like ["a_string", 13]
    pub data: String,
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
                    contract: contract,
                    action_name: action_name,
                    data: data,
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
            contract,
            action_name,
            data,
        }
    }

    #[cfg(feature = "std")]
    pub fn from_str(
        contract: String,
        action_name: String,
        action_transfer: ActionTransfer,
    ) -> crate::Result<Self> {
        let data = serde_json::to_string(&action_transfer).unwrap();
        Ok(Action {
            contract: contract,
            action_name: action_name,
            data: data,
        })
    }

    #[cfg(feature = "std")]
    pub fn transfer<T: AsRef<str>>(
        from: String,
        to: String,
        quantity: String,
        memo: T,
    ) -> crate::Result<Action> {
        let action_transfer = ActionTransfer::from_str(from, to, quantity, memo)?;
        Action::from_str(
            String::from("token.iost"),
            String::from("transfer"),
            action_transfer,
        )
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
            self.contract, self.action_name, self.data,
        )
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Read, Write, NumberBytes, Default, SerializeData)]
#[iost_root_path = "crate"]
pub struct ActionTransfer {
    pub tokenType: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub memo: String,
}

impl ActionTransfer {
    pub fn new(tokenType: String, from: String, to: String, amount: String, memo: String) -> Self {
        ActionTransfer {
            tokenType,
            from,
            to,
            amount,
            memo,
        }
    }

    pub fn from_str<T: AsRef<str>>(
        from: String,
        to: String,
        amount: String,
        memo: T,
    ) -> crate::Result<Self> {
        Ok(ActionTransfer {
            tokenType: String::from("iost"),
            from,
            to,
            amount,
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
            contract: contract,
            action_name: action_name,
            data: data,
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            contract: s.to_string(),
            action_name: s.to_string(),
            data: s.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action() {
        let action = Action {
            contract: "iost".to_string(),
            action_name: "iost".to_string(),
            data: "".to_string(),
        };
        dbg!(action);
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

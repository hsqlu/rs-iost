use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::Error::{BytesReadError, InvalidPublisherSignature, InvalidSignature};
use crate::{
    Action, AmountLimit, NumberBytes, Read, ReadError, SerializeData, Signature, Write, WriteError,
};
use chrono::{SecondsFormat, TimeZone, Utc};
use keys::algorithm;
use serde::Deserializer;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::slice::Iter;

#[derive(Clone, Default, Debug, SerializeData)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[iost_root_path = "crate"]
pub struct Tx {
    /// Time of transaction. Unixepoch start in nanoseconds
    pub time: i64,
    /// Transaction expiration time. Unixepoch starts in nanoseconds. If the chunk node does not receive the transaction until after the expiration time, it will not execute
    pub expiration: i64,
    /// GAS multiplying rate. This transaction shall be paid according to the gas ratio of the default gas. The higher the multiplier, the higher the priority. The reasonable value range is [1.0, 100.0]
    pub gas_ratio: f64,
    /// The maximum allowed gas of the transaction, with a minimum setting of 50000
    pub gas_limit: f64,
    /// Used in delayed transactions. The number of nanoseconds to delay execution. Non delayed transaction set to 0
    pub delay: i64,
    /// Network ID
    pub chain_id: u32,
    /// Specific call in transaction
    pub actions: Vec<Action>,
    /// Token restrictions on transactions. You can specify multiple tokens and a corresponding number limit. If the transaction exceeds these limits, execution fails
    pub amount_limit: Vec<AmountLimit>,
    /// ID of the transaction sender
    // #[serde(skip)]
    // #[serde(deserialize_with="parse_color")]
    #[serde(default)]
    pub publisher: String,
    /// Publisher's signature. The signing process is as follows. Publisher can provide multiple signatures with different permissions. You can refer to the documentation of the permission system
    // #[serde(skip)]
    #[serde(default)]
    // #[serde(deserialize_with="parse_color")]
    pub publisher_sigs: Vec<Signature>,
    /// Signer ID other than publisher. It can be empty.
    pub signers: Vec<String>,
    /// Signature of signers. Each signer can have one or more signatures, so the length is not less than the length of signers
    pub signatures: Vec<Signature>,
}

impl NumberBytes for Tx {
    #[inline]
    fn num_bytes(&self) -> usize {
        48 + self.signers.num_bytes()
            + self.signers.len() * 4
            + self.actions.num_bytes()
            + self.actions.len() * 4
            + self.amount_limit.num_bytes()
            + self.amount_limit.len() * 4
            + self.signatures.num_bytes()
            + self.signatures.len() * 4
    }
}

impl Read for Tx {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let time = i64::read(bytes, pos)?;
        let expiration = i64::read(bytes, pos)?;
        let gas_ratio = f64::read(bytes, pos)?;
        let gas_limit = f64::read(bytes, pos)?;
        let delay = i64::read(bytes, pos)?;
        let chain_id = u32::read(bytes, pos)?;

        // reserved field, default len 0 for now.
        let _reserved = i32::read(bytes, pos)?;

        let signers_capacity = usize::read(bytes, pos)?;
        let mut signers: Vec<String> = Vec::new();
        signers.resize(signers_capacity, String::default());
        for item in &mut signers {
            let _size = usize::read(bytes, pos)?;
            let r = String::read(bytes, pos)?;
            *item = r;
        }

        let actions_capacity = usize::read(bytes, pos)?;
        let mut actions: Vec<Action> = Vec::new();
        actions.resize(actions_capacity, Action::default());

        for item in &mut actions {
            let _size = usize::read(bytes, pos)?;
            let r = Action::read(bytes, pos)?;
            *item = r;
        }

        let amount_limits_capacity = usize::read(bytes, pos)?;
        let mut amount_limit: Vec<AmountLimit> = Vec::new();
        amount_limit.resize(amount_limits_capacity, AmountLimit::default());

        for item in &mut amount_limit {
            let _size = usize::read(bytes, pos)?;
            let r = AmountLimit::read(bytes, pos)?;
            *item = r;
        }

        let signatures_capacity = usize::read(bytes, pos)?;
        let mut signatures: Vec<Signature> = Vec::new();
        signatures.resize(signatures_capacity, Signature::default());

        for item in &mut signatures {
            let _size = usize::read(bytes, pos)?;
            let r = Signature::read(bytes, pos)?;
            *item = r;
        }

        Ok(Tx {
            time,
            expiration,
            gas_ratio,
            gas_limit,
            delay,
            chain_id,
            actions,
            signers,
            amount_limit,
            signatures,
            publisher: "".to_string(),
            publisher_sigs: vec![],
        })
    }
}

impl Write for Tx {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.time.clone().write(bytes, pos);
        self.expiration.clone().write(bytes, pos);
        let mut ratio = (self.gas_ratio * 100.0) as i64;
        ratio.write(bytes, pos);
        let mut limit = (self.gas_limit * 100.0) as i64;
        limit.write(bytes, pos);
        self.delay.clone().write(bytes, pos);
        self.chain_id.clone().write(bytes, pos);

        // reserved field
        0_i32.write(bytes, pos);

        self.signers.len().write(bytes, pos)?;
        expand::<String>(self.signers.iter(), bytes, pos);
        self.actions.len().write(bytes, pos);
        expand::<Action>(self.actions.iter(), bytes, pos);
        self.amount_limit.len().write(bytes, pos);
        expand::<AmountLimit>(self.amount_limit.iter(), bytes, pos);
        self.signatures.len().write(bytes, pos);
        expand::<Signature>(self.signatures.iter(), bytes, pos)
    }
}

fn expand<T>(x: Iter<T>, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError>
where
    T: NumberBytes + Write,
{
    for item in x {
        item.num_bytes().write(bytes, pos)?;
        item.write(bytes, pos)?;
    }
    Ok(())
}

impl Tx {
    pub fn new() -> Self {
        Tx {
            time: 0,
            expiration: 0,
            gas_ratio: 0.0,
            gas_limit: 0.0,
            delay: 0,
            chain_id: 0,
            actions: vec![],
            amount_limit: vec![],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![],
        }
    }

    pub fn from_action(actions: Vec<Action>) -> Self {
        // let time = Utc::now().timestamp();
        let time: i64 = 0;
        let expiration = time + 90000000000;

        Tx {
            time,
            expiration,
            gas_ratio: 1000000.0,
            gas_limit: 1.0,
            delay: 0,
            chain_id: 0,
            actions,
            amount_limit: vec![AmountLimit {
                token: String::from("*"),
                value: String::from("unlimited"),
            }],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![],
        }
    }

    fn write(&self, bytes: &mut [u8], pos: &mut usize, with_sign: bool) -> Result<(), WriteError> {
        self.time.clone().write(bytes, pos);
        self.expiration.clone().write(bytes, pos);
        let mut ratio = (self.gas_ratio * 100.0) as i64;
        ratio.write(bytes, pos);
        let mut limit = (self.gas_limit * 100.0) as i64;
        limit.write(bytes, pos);
        self.delay.clone().write(bytes, pos);
        self.chain_id.clone().write(bytes, pos);

        // reserved field
        0_i32.write(bytes, pos);

        self.signers.len().write(bytes, pos)?;
        expand::<String>(self.signers.iter(), bytes, pos);
        self.actions.len().write(bytes, pos);
        expand::<Action>(self.actions.iter(), bytes, pos);
        self.amount_limit.len().write(bytes, pos);
        expand::<AmountLimit>(self.amount_limit.iter(), bytes, pos);
        if with_sign {
            self.signatures.len().write(bytes, pos);
            expand::<Signature>(self.signatures.iter(), bytes, pos);
        }
        Ok(())
    }

    pub fn customized_to_serialize_data(&self, with_sign: bool) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0, with_sign)
            .map_err(crate::Error::BytesWriteError)
            .unwrap();
        Ok(data.to_vec())
    }

    pub fn sign(
        &mut self,
        account_name: String,
        sign_algorithm: &str,
        sec_key: &[u8],
    ) -> Result<(), WriteError> {
        self.publisher = account_name;

        if self.publisher_sigs.len() == 0 {
            let tx_bytes = self.customized_to_serialize_data(true).unwrap();
            // create a SHA3-256 object
            let mut hasher = Sha3_256::new();
            hasher.input(tx_bytes);
            let result = hasher.result();
            self.publisher_sigs =
                vec![Signature::sign(result.as_ref(), algorithm::ED25519, sec_key).unwrap()];
        }
        Ok(())
    }

    pub fn verify(&self) -> bool {
        for signature in &self.signatures {
            let tx_bytes = self.customized_to_serialize_data(false).unwrap();
            let mut hasher = Sha3_256::new();
            hasher.input(tx_bytes);
            let result = hasher.result();
            if !signature.verify(result.as_slice()) {
                // Err(InvalidSignature())
                return false;
            }
        }
        for publisher_sig in &self.publisher_sigs {
            let tx_bytes = self.customized_to_serialize_data(true).unwrap();
            let mut hasher = Sha3_256::new();
            hasher.input(tx_bytes);
            let result = hasher.result();
            if !publisher_sig.verify(result.as_slice()) {
                return false;
            }
        }
        true
    }
}

fn parse_color<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tx() {
        let mut tx = Tx {
            time: 1544013436179000000,
            expiration: 1544013526179000000,
            gas_ratio: 100.0,
            gas_limit: 123400.0,
            delay: 0,
            chain_id: 0,
            actions: vec![Action {
                contract: "cont".to_string(),
                action_name: "abi".to_string(),
                data: "[]".to_string(),
            }],
            amount_limit: vec![AmountLimit {
                token: "iost".to_string(),
                value: "123".to_string(),
            }],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec!["abc".to_string()],

            signatures: vec![],
        };
        let data: Vec<u8> = tx.to_serialize_data().unwrap();

        // let s = String::from_utf8(data.clone());
        dbg!(hex::encode(data.as_slice()));
        // create a SHA3-256 object
        let mut hasher = Sha3_256::new();

        // write input message
        // let data = result.unwrap();
        hasher.input(data);
        let result = hasher.result();
        // dbg!(result.as_slice());
        // assert_eq!(
        //     "93c24341c06cd7a23023d278dd044bf736730ac5e32d432aff05a00ac3df85f8",
        //     hex::encode(result.as_slice())
        // );
    }

    #[test]
    fn should_tx_sign_be_ok() {
        let mut tx = Tx {
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1.0,
            gas_limit: 500000.0,
            delay: 0,
            chain_id: 1024,
            actions: vec![ Action {
                contract: "token.iost".to_string(),
                action_name: "transfer".to_string(),
                data: "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]".to_string()
            }],
            amount_limit: vec![ AmountLimit {
                token: "*".to_string(),
                value: "unlimited".to_string()
            }],
            publisher: "".to_string(),
            publisher_sigs: vec![],
            signers: vec![],
            signatures: vec![]
        };

        let sec_key = base64::decode("gkpobuI3gbFGstgfdymLBQAGR67ulguDzNmLXEJSWaGUNL5J0z5qJUdsPJdqm+uyDIrEWD2Ym4dY9lv8g0FFZg==").unwrap();
        tx.sign(
            "testaccount".to_string(),
            algorithm::ED25519,
            sec_key.as_slice(),
        );
        // let tx_string = serde_json::to_string_pretty(&tx).unwrap();
        // dbg!(&tx_string);
        assert!(tx.verify());

        let tx_str = r#"
        {
            "time": 1544709662543340000,
            "expiration": 1544709692318715000,
            "gas_ratio": 1,
            "gas_limit": 500000,
            "delay": 0,
            "chain_id": 1024,
            "signers": [],
            "actions": [{
                "contract": "token.iost",
                "action_name": "transfer",
                "data": "[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]"
            }],
            "amount_limit": [{
                "token": "*",
                "value": "unlimited"
            }],
            "signatures": [],
            "publisher": "testaccount",
            "publisher_sigs": [{
                "algorithm": "ED25519",
                "public_key": "lDS+SdM+aiVHbDyXapvrsgyKxFg9mJuHWPZb/INBRWY=",
                "signature": "/K1HM0OEbfJ4+D3BmalpLmb03WS7BeCz4nVHBNbDrx3/A31aN2RJNxyEKhv+VSoWctfevDNRnL1kadRVxSt8CA=="
            }]
        }
        "#;
        let tx_struct: Result<Tx, _> = serde_json::from_str(tx_str);

        assert!(tx_struct.is_ok());
        if let Ok(expected_tx) = tx_struct {
            // let result = tx.to_serialize_data();
            // assert!(result.is_ok());
            assert_eq!(tx.publisher_sigs.len(), 1);
            assert_eq!(tx.publisher_sigs.len(), expected_tx.publisher_sigs.len());
            assert_eq!(
                tx.publisher_sigs[0].public_key,
                expected_tx.publisher_sigs[0].public_key
            );
            assert_eq!(
                tx.publisher_sigs[0].signature,
                expected_tx.publisher_sigs[0].signature
            );
        }
    }
}

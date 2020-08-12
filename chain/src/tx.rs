use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::{
    Action, AmountLimit, NumberBytes, Read, ReadError, SerializeData, Signature, Write, WriteError,
};
use chrono::{SecondsFormat, TimeZone, Utc};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
// #[iost_root_path = "crate"]
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
    pub publisher: String,
    /// Publisher's signature. The signing process is as follows. Publisher can provide multiple signatures with different permissions. You can refer to the documentation of the permission system
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
            + self.actions.num_bytes()
            + self.amount_limit.num_bytes()
            + self.signatures.num_bytes()
    }
}

impl Read for Tx {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let time_bits = i64::read(bytes, pos)?;
        let time = i64::from(time_bits);
        let expiration_bits = i64::read(bytes, pos)?;
        let expiration = i64::from(expiration_bits);
        let gas_ratio_bits = f64::read(bytes, pos)?;
        let gas_ratio = f64::from(gas_ratio_bits) / 100.0;
        let gas_limit_bits = f64::read(bytes, pos)?;
        let gas_limit = f64::from(gas_limit_bits) / 100.0;
        let delay_bits = i64::read(bytes, pos)?;
        let delay = i64::from(delay_bits);
        let chain_id_bits = u32::read(bytes, pos)?;
        let chain_id = u32::from(chain_id_bits);
        let signers_capacity = usize::read(bytes, pos)?;
        let mut signers = Vec::new();
        signers.resize(signers_capacity, String::default());

        for item in &mut signers {
            let r = String::read(bytes, pos)?;
            *item = r;
        }

        let actions_capacity = usize::read(bytes, pos)?;
        let mut actions = Vec::new();
        actions.resize(actions_capacity, Action::default());

        for item in &mut actions {
            let r = Action::read(bytes, pos)?;
            *item = r;
        }

        let actions_capacity = usize::read(bytes, pos)?;
        let mut actions = Vec::new();
        actions.resize(actions_capacity, Action::default());

        for item in &mut actions {
            let r = Action::read(bytes, pos)?;
            *item = r;
        }

        let amount_limits_capacity = usize::read(bytes, pos)?;
        let mut amount_limit = Vec::new();
        amount_limit.resize(amount_limits_capacity, AmountLimit::default());

        for item in &mut amount_limit {
            let r = AmountLimit::read(bytes, pos)?;
            *item = r;
        }

        let signatures_capacity = usize::read(bytes, pos)?;
        let mut signatures = Vec::new();
        signatures.resize(signatures_capacity, Signature::default());

        for item in &mut signatures {
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
        // let mut time: i64 = self.time;
        // time.write(bytes, pos);
        self.expiration.clone().write(bytes, pos);
        let mut ratio = (self.gas_ratio * 100.0) as i64;
        ratio.write(bytes, pos);
        let mut limit = (self.gas_limit * 100.0) as i64;
        limit.write(bytes, pos);
        self.delay.clone().write(bytes, pos);
        self.chain_id.clone().write(bytes, pos);
        0u32.write(bytes, pos);
        self.signers.as_slice().write(bytes, pos);
        self.actions.as_slice().write(bytes, pos);
        self.amount_limit.as_slice().write(bytes, pos);
        self.signatures.as_slice().write(bytes, pos)
    }
}

impl SerializeData for Tx {
    fn to_serialize_data(&self) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0)
            .map_err(crate::Error::BytesWriteError)?;
        Ok(data.to_vec())
    }
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

    // t.Publisher = s.accountName
    // if len(t.PublisherSigs) == 0 {
    // signAlgorithm := GetSignAlgoByName(signAlgo)
    // txHashBytes := common.Sha3(txToBytes(t, true))
    // publishSig := &rpcpb.Signature{
    // Algorithm: rpcpb.Signature_Algorithm(signAlgorithm),
    // Signature: signAlgorithm.Sign(txHashBytes, s.keyPair.Seckey),
    // PublicKey: signAlgorithm.GetPubkey(s.keyPair.Seckey),
    // }
    // t.PublisherSigs = []*rpcpb.Signature{publishSig}
    // }
    // return t, nil
    pub fn sign_tx(&mut self, kps: Vec<Signature>, signer: Signature) -> Result<(), WriteError> {
        self.signatures.push(signer);

        for kp in kps {
            // let sig = kp.sign(tx.publish_hash());
        }
        // self.publisher = account_name;
        if self.publisher_sigs.len() == 0 {
            let mut bytes: Vec<u8> = Vec::new();
            bytes.resize(self.num_bytes(), 0);
            self.write(&mut *bytes, &mut (0 as usize))?;
            // sha3::digest(bytes);

            let mut _hasher = Sha3_256::new();
            // hasher
            // hasher::input(bytes.as_slice());
            // let result = hasher.result();
            self.publisher_sigs = vec![Signature {
                algorithm: "".to_string(),
                signature: "".to_string(),
                public_key: "".to_string(),
            }]
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_send_tx_deserialize_should_be_ok() {
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
        if let Ok(tx) = tx_struct {
            let tx_string = serde_json::to_string_pretty(&tx).unwrap();
            dbg!(&tx_string);
            // assert_eq!(&tx_str, &tx_string);
        }
    }
}

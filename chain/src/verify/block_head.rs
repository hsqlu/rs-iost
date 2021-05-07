use base64;
use codec::{Decode, Encode};

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::from_utf8;
use keys::algorithm;
use keys::algorithm::Algorithm;

use crate::spv::{Head, Sign};
use crate::Error::IOSTBlockVerifyError;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct BlockHead {
    pub version: i64,

    pub parent_hash: Vec<u8>,

    pub tx_merkle_hash: Vec<u8>,

    pub tx_receipt_merkle_hash: Vec<u8>,

    pub info: Vec<u8>,

    pub number: i64,

    pub witness: Vec<u8>,

    pub time: i64,

    pub hash: Vec<u8>,

    pub algorithm: u8,
    pub sig: Vec<u8>,
    pub pub_key: Vec<u8>,
}

impl BlockHead {
    pub fn parse_head(&self) -> Head {
        let mut head = Head {
            version: self.version,
            parent_hash: parse_and_decode(self.parent_hash.clone()),
            tx_merkle_hash: parse_and_decode(self.tx_merkle_hash.clone()),
            tx_receipt_merkle_hash: parse_and_decode(self.tx_receipt_merkle_hash.clone()),
            info: parse_and_decode(self.info.clone()),
            number: self.number,
            witness: "".to_string(),
            time: self.time,
        };
        head.witness = core::str::from_utf8(self.witness.as_slice())
            .unwrap()
            .to_string();
        return head;
    }

    pub fn parse_sign(&self) -> Sign {
        Sign {
            algorithm: self.algorithm,
            sig: from_utf8(self.sig.as_slice()).unwrap().to_string(),
            pub_key: from_utf8(self.pub_key.as_slice()).unwrap().to_string(),
        }
    }

    pub fn verify_self(&self) -> bool {
        let head = self.parse_head();
        let sign = self.parse_sign();
        return head.verify(sign);
    }
}

fn parse_and_decode(input: Vec<u8>) -> Vec<u8> {
    let result = core::str::from_utf8(input.as_slice()).unwrap().to_string();
    let res = base64::decode(result).unwrap();
    res.to_vec()
}

#[cfg(test)]
mod test {

    use crate::verify::BlockHead;
    use alloc::vec;
    use alloc::vec::Vec;
    use base64;

    const TEST_BLOCKS: Vec<BlockHead> = vec![];

    fn init() {
        TEST_BLOCKS.push(BlockHead {
            version: 1,
            parent_hash: base64_decode("ayIjoV383UIPRxlXM5AHtNmboqKZXZBhNl6rElpuCRA="),
            tx_merkle_hash: base64_decode("YghPcRrtsuJ/8AqXeK8DdFtOl8j9lyKeTT1rPpp/wBQ="),
            tx_receipt_merkle_hash: base64_decode("vSGIHJPnI6eWrJ5Oh6AZ/fe2DoIF35WY94kCwW2bPn4="),
            info: base64_decode("eyJtb2RlIjowLCJ0aHJlYWQiOjAsImJhdGNoIjpudWxsfQ=="),
            number: 102492000,
            // 102504000
            witness: "G5DPSoGy4J4y5ZzGQ5uPXbddJFCyzBzva2r5XjFSsNVa".as_bytes().to_vec(),
            time: 1603139621500090226,
            hash: vec!(),
            algorithm: 2,
            sig: "BXoieBOEDU6/u5wsPvEjOAhR6es9kPOV4fObcQb0/lw1QUx5MpWut09McJXq75Rh4vt1eYv+SqF9CfTJVixPBQ==".as_bytes().to_vec(),
            pub_key: "3/OiFQp5j4y3AOAE5mfqImSIrdQHNLm0KqrEmzBJpw0=".as_bytes().to_vec()
        });

        TEST_BLOCKS.push(BlockHead {
            version: 1,
            parent_hash: base64_decode("82apIKeM19IFQ9xxHJL1UGLBrh8dXgMDjCvq9gjn/z4="),
            tx_merkle_hash: base64_decode("2cXAjBtW6WszInZWAtG+iZCOGLw9Gf4U+hKd+ML+ygk="),
            tx_receipt_merkle_hash: base64_decode("su1H+XngQyXfe0FlZ2BhtnBsMhFRUfB1o/eUwkHmrVM="),
            info: base64_decode("eyJtb2RlIjowLCJ0aHJlYWQiOjAsImJhdGNoIjpudWxsfQ=="),
            number: 102493200,
            // 102504000
            witness: "C4Qcz8TqZdRf8xsMLwzv894UMNZ28dRPGLnHpog3bBrJ".as_bytes().to_vec(),
            time: 1603140240000166982,
            hash: vec!(),
            algorithm: 2,
            sig: "FpTc0FovW8npBq3MpkYWo5D5C1Z40s98sotAPZ8Lib2JsZbcRqIIIIIrjSioKw6kbnD3S13iCVQS0iv7yA6YBw==".as_bytes().to_vec(),
            pub_key: "pFABnmUusdVX33Rq0VmBouJT8Mr+jnGM1Keb1/80YGM=".as_bytes().to_vec()
        });
    }

    #[test]
    fn verify_block_head_should_work() {
        for h in TEST_BLOCKS.iter() {
            // dbg!(h.parse_head());
            // dbg!(h.parse_sign());
            assert!(h.verify_self());
        }
    }

    fn base64_decode(s: &str) -> Vec<u8> {
        return s.as_bytes().to_vec();
    }

    #[test]
    fn test_base64_decode() -> () {
        let input = "qy5kSuCpYMQ+MFo0NZ4v29j9vjg7rqp5Mgw6ui/Fg/I=";
        // let input = "75xEVVZQRiqxozORz0DZ4OAG9VKCKSEpc2LhgsSUKZ4=";
        let res = base64::decode(input).unwrap();
        let r2 = bs58::encode(res).into_string();
        // let result = core::str::from_utf8(res.as_slice()).unwrap().to_string();
        dbg!(r2);

        let x = "CXDkEdeKQArGf6HUwzDdz7gvtBWapDwqcMsXRgcB1oUV";

        let re = bs58::decode(x).into_vec().unwrap();
        dbg!(re);

        // let sec_key = bs58::decode("xjggJ3TrLXz7qEwrGG3Rc4Fz59imjixhXpViq9W7Ncx")
        //     .into_vec()
        //     .unwrap();
    }
}

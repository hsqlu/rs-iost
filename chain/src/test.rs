use crate::time_point::TimePoint;
use crate::{Action, AmountLimit, NumberBytes, Read, Result, SerializeData, Tx, Write};
use base58::{FromBase58, ToBase58};
use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use keys::algorithm;

#[test]
fn it_works() {
    let action = Action::new(
        String::from("token.iost").into_bytes(),
        String::from("transfer").into_bytes(),
        String::from(r#"["iost","admin","lispczz","10.12034123",""]"#).into_bytes(),
    );
    let result: Result<Vec<u8>> = action.to_serialize_data();
    assert!(result.is_ok());

    // let amount_limit = AmountLimit {
    //     token: "*".to_string(),
    //     value: "unlimited".to_string(),
    // };
    // let time = Utc::now().timestamp_nanos();
    // let expiration = time + Duration::seconds(10000).num_nanoseconds().unwrap();
    //
    // let tx = Tx {
    //     time: 1597135689684857000,
    //     expiration: 1597135779684857000,
    //     gas_ratio: 1.0,
    //     gas_limit: 1000000.0,
    //     delay: 0,
    //     chain_id: 1024,
    //     actions: vec![action.clone()],
    //     amount_limit: vec![amount_limit],
    //     publisher: "".to_string(),
    //     publisher_sigs: vec![],
    //     signers: vec![],
    //     signatures: vec![],
    // };
    // let mut data = vec![0u8; tx.num_bytes()];
    // tx.write(&mut data, &mut 0)
    //     .map_err(crate::Error::BytesWriteError).unwrap();
    // dbg!(data.to_vec());
    // dbg!(result);
}

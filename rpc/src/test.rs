#[cfg(test)]
extern crate alloc;

use alloc::vec::Vec;
use iost_chain::spv::{Block as RawBlock, Verify, VOTE_INTERVAL};

use iost_chain::SerializeData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    /// error code status
    pub code: i32,
    /// error message
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    ///Error request message
    Reqwest(reqwest::Error),
    ///Error response message
    ErrorMessage(ErrorMessage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawBlockByNumber {
    pub status: String,
    pub block: RawBlock,
}

async fn get_raw_block_by_number(
    domain: &str,
    number: i32,
    complete: bool,
) -> Result<RawBlockByNumber, Error> {
    let url = format!("{}/getRawBlockByNumber/{}/{}", domain, number, complete);

    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req
            .json::<RawBlockByNumber>()
            .await
            .map_err(Error::Reqwest)?;
        Ok(rsp)
    } else {
        let rsp = req.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(rsp))
    }
}

#[tokio::test]
async fn iost_block_check_should_be_ok() {
    let block_number = 102492000;
    let response = get_raw_block_by_number("http://api.iost.io", block_number, true).await;
    dbg!(&response);
    // assert!(response.is_ok());
    let block: RawBlock = response.unwrap().block;
    // let data = block.head.to_serialize_data().unwrap();
    let mut v: Verify = iost_chain::spv::init(&block).unwrap();

    let mut starter = 60 + block_number;
    let response = get_raw_block_by_number("http://api.iost.io", starter, true).await;
    let start_block = response.unwrap().block;
    let mut block_list: Vec<RawBlock> = Vec::new();
    for i in 1..108 {
        let response = get_raw_block_by_number("http://api.iost.io", starter + i, true).await;
        block_list.push(response.unwrap().block);
        dbg!(&block_list[block_list.len() - 1]);
    }
    let result = v.check_block(&start_block, block_list);
    assert!(result.is_ok());
    //
    // let mut new_starter = 102493200;
    // let mut new_block_list: Vec<RawBlock> = Vec::new();
    //
    // let response = get_raw_block_by_number("http://api.iost.io", new_starter, true).await;
    // let update = response.unwrap().block;
    // for i in 1..108 {
    //     let response = get_raw_block_by_number("http://api.iost.io", new_starter + i, true).await;
    //     new_block_list.push(response.unwrap().block);
    //     dbg!(&new_block_list[new_block_list.len() - 1]);
    // }
    //
    // if new_starter as i64 % VOTE_INTERVAL == 0 {
    //     let result = v.update_epoch(&update, new_block_list);
    //     assert!(result.is_ok());
    // }
}

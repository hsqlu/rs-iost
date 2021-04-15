pub mod block_head;

pub use self::block_head::*;

use crate::spv::VOTE_INTERVAL;
use crate::Error::{IOSTBlockWitnessError, IOSTInvalidBlockSignature};
use crate::Result;
use alloc::format;
use alloc::vec::Vec;

// pub fn check_witness(block: &BlockHead, witness_blocks: Vec<BlockHead>) -> Result<()> {
//     if let Err(_) = block.verify_self() {
//         return Err(IOSTInvalidBlockSignature());
//     }
//
//     for b in witness_blocks.iter() {
//         if let Err(_) = b.verify_self() {
//             return Err(IOSTInvalidBlockSignature());
//         }
//     }
//
//     let block_number: i64 = block.number;
//     let mut current_epoch_start_block: i64 = 0;
//
//     if block_number % VOTE_INTERVAL == 0 {
//         current_epoch_start_block = block_number - VOTE_INTERVAL
//     } else {
//         current_epoch_start_block = block_number / VOTE_INTERVAL * VOTE_INTERVAL
//     }
//
//     Ok(())
//
//     // match v.epoch_producer.get(&current_epoch_start_block) {
//     //     Some(pending_list) => {
//     //         let mut valid_witness_count = 0;
//     //         let mut valid_witness: BTreeMap<String, bool> = BTreeMap::new();
//     //
//     //         let mut parent_hash = block.hash();
//     //         let mut parent_block_number = block.number;
//     //
//     //         for b in witness_blocks.iter() {
//     //             // let block_parent_hash = &b.head.parent_hash;
//     //             if parent_hash.as_slice() != b.head.parent_hash.as_slice() {
//     //                 return Err(IOSTBlockWitnessError(format!(
//     //                     "invalid block hash at block {}",
//     //                     b.head.number
//     //                 )));
//     //             }
//     //             if parent_block_number + 1 != b.head.number {
//     //                 return Err(IOSTBlockWitnessError(format!(
//     //                     "invalid block number at block {}",
//     //                     b.head.number
//     //                 )));
//     //             }
//     //
//     //             match valid_witness.get(&b.head.witness) {
//     //                 None => {
//     //                     for produce in v.current_producer.iter() {
//     //                         if produce.eq(&b.head.witness) {
//     //                             valid_witness.insert(produce.to_string(), true);
//     //                             valid_witness_count = valid_witness_count + 1;
//     //                             break;
//     //                         }
//     //                     }
//     //                 }
//     //                 _ => {}
//     //             }
//     //             parent_block_number = b.head.number;
//     //             parent_hash = b.head.hash();
//     //         }
//     //         if valid_witness_count < 12 {
//     //             return Err(IOSTBlockWitnessError(format!(
//     //                 "valid witness not enough {}",
//     //                 valid_witness_count
//     //             )));
//     //         }
//     //         Ok(())
//     //     }
//     //     None => Err(IOSTBlockWitnessError(format!(
//     //         "cannot update producer list at block {}: cannot find producer info of previous epoch",
//     //         block_number
//     //     ))),
//     // }
// }

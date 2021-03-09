use codec::Encode;
use core::marker::PhantomData;
use iost_chain::IostAction;
use once_cell::sync::Lazy; // sync::OnceCell is thread-safe
use once_cell::sync::OnceCell; // sync::OnceCell is thread-safe
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use subxt::{
    system::{AccountStoreExt, System, SystemEventsDecoder},
    Call, Client, DefaultNodeRuntime as BifrostRuntime, Error as SubxtErr, PairSigner,
};

#[derive(Clone, Debug)]
pub enum Error {
    NullPtr(String),
    CStrConvertError,
    PublicKeyError,
    SignatureError,
    WrongSudoSeed,
    SubxtError(&'static str),
}

static BIFROST_RPC_CLIENT: Lazy<Arc<Mutex<subxt::ClientBuilder<BifrostRuntime>>>> = {
    Lazy::new(move || {
        let builder: subxt::ClientBuilder<BifrostRuntime> = subxt::ClientBuilder::new();
        Arc::new(Mutex::new(builder))
    })
};

async fn global_client(
    url: &str,
) -> Result<&'static Mutex<subxt::Client<BifrostRuntime>>, crate::Error> {
    static INSTANCE: OnceCell<Mutex<subxt::Client<BifrostRuntime>>> = OnceCell::new();
    let builder = subxt::ClientBuilder::new()
        .set_url(url)
        .build()
        .await
        .map_err(|_| crate::Error::SubxtError("failed to create subxt client"))?;
    Ok(INSTANCE.get_or_init(|| Mutex::new(builder)))
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ProveActionCall<T: BridgeIost> {
    action: IostAction,
    trx_id: Vec<u8>,
    pub _runtime: PhantomData<T>,
}

#[subxt::module]
pub trait BridgeIost: System {}

impl BridgeIost for BifrostRuntime {}

async fn call(
    client: Client<BifrostRuntime>,
    action_data: &str,
    trx_id: Vec<u8>,
) -> Result<String, crate::Error> {
    let signer = Pair::from_string("//Alice".as_ref(), None)
        .map_err(|_| crate::Error::WrongSudoSeed)
        .unwrap();
    let mut signer = PairSigner::<BifrostRuntime, Pair>::new(signer);
    static atomic_nonce: AtomicU32 = AtomicU32::new(0);
    let current_nonce = client
        .account(&signer.signer().public().into(), None)
        .await
        .map_err(|_| crate::Error::WrongSudoSeed)?
        .nonce;

    // 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@bifrost:IOST
    let result_action: IostAction = serde_json::from_str(action_data).unwrap();

    let call = ProveActionCall::<BifrostRuntime> {
        action: result_action,
        trx_id: trx_id,
        _runtime: PhantomData,
    };

    match client.submit(call.clone(), &signer).await {
        Ok(trx_id) => Ok(trx_id.to_string()),
        Err(SubxtErr::Rpc(e)) => {
            let trx_id = client.submit(call, &signer).await.map_err(|e| {
                println!("error is: {:?}", e.to_string());
                crate::Error::SubxtError("failed to commit this transaction")
            })?;
            Ok(trx_id.to_string())
        }
        _ => Err(crate::Error::SubxtError(
            "failed to commit this transaction",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use subxt::{
        system::{System, SystemEventsDecoder},
        Call, Client, DefaultNodeRuntime as BifrostRuntime, PairSigner,
    };

    const TO_BIFROST: &str = r#"
        {
            "contract": "token.iost",
            "action_name": "transfer",
            "data": "[\"iost\", \"lispczz5\", \"bifrost\", \"20\", \"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@bifrost:IOST\"]"
        }
        "#;

    const TO_IOST: &str = r#"
        {
            "contract": "token.iost",
            "action_name": "transfer",
            "data": "[\"iost\", \"bifrost\", \"lispczz5\", \"9\", \"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@bifrost:IOST\"]"
        }
        "#;

    async fn build_client() -> Client<BifrostRuntime> {
        subxt::ClientBuilder::new()
            .set_url("ws://127.0.0.1:9944")
            .build()
            .await
            .map_err(|e| dbg!(e))
            .unwrap()
    }

    #[test]
    fn debug_iost_action() {
        let result_action: IostAction = serde_json::from_str(TO_IOST).unwrap();
        dbg!(result_action);
    }

    #[test]
    fn it_works() {
        let memo: String =
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@bifrost:IOST".to_string();
        let split_memo = memo
            .as_str()
            .split(|c| c == '@' || c == ':')
            .collect::<Vec<_>>();
        assert_eq!(split_memo.len(), 3);

        let _a = match split_memo[2] {
            "IOST" => {
                dbg!("split_memo[2]");
                ""
            }
            _ => "--",
        };
    }

    #[test]
    fn deposit_iost_to_bifrost() {
        let result = futures::executor::block_on(async move {
            crate::call(build_client().await, TO_BIFROST, "".as_bytes().to_vec()).await
        });

        dbg!(result.is_ok());
    }

    #[test]
    fn transfer_to_iost() {
        let result = futures::executor::block_on(async move {
            crate::call(
                build_client().await,
                TO_IOST,
                "GQxsjgWVKjXcW67ETQyEYEpaM4TH3J933AGefdueezW8"
                    .as_bytes()
                    .to_vec(),
            )
            .await
        });

        dbg!(result.is_ok());
    }
}

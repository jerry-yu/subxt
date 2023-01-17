// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

// This example showcases working with get event log one block by one

use subxt::{
    OnlineClient,
    PolkadotConfig,
};
use subxt::events::StaticEvent;
use subxt::ext::codec::{Decode, Encode};
use subxt::utils::AccountId32;

#[derive(Decode, Encode, Debug)]
pub struct TransferEvent {
    from: AccountId32,
    to: AccountId32,
    amount: u128,
}

impl StaticEvent for TransferEvent {
    const PALLET: &'static str = "Balances";
    const EVENT: &'static str = "Transfer";
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    const LIMIT :u32 = 50;
    // in production mod ,block_num should persistence 
    let mut block_num = 1u32;
    while block_num < LIMIT {
        let block_hash = api.rpc().block_hash(Some(block_num.into())).await?;
        
        if let Some(block_hash) = block_hash {
            println!("block_hash {:?}",block_hash);
            let events = api.events().at(Some(block_hash)).await?;
            let events = events.iter().filter_map(|ev| ev.ok());
            for ev in events {
                if let Some(ev) = ev.as_event::<TransferEvent>().ok().flatten() {
                    println!("event -- {:?}",ev)
                }
            }
            block_num+=1;
        } else {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }
    Ok(())
}

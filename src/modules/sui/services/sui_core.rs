use std::str::FromStr;

use anyhow::Result;
use fastcrypto::{
    ed25519::{Ed25519KeyPair, Ed25519PrivateKey},
    traits::KeyPair,
};

use sui_keys::keystore::InMemKeystore;
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiRawData},
    SuiClientBuilder,
};
use sui_types::{
    base_types::SuiAddress,
    crypto::{get_key_pair, SuiKeyPair},
    object::Owner,
};

const USDC_TYPE: &str =
    "0xa1ec7fc00a6f40db9693ad1415d0c193ad3906494428cf252621037bd7117e29::usdc::USDC";

pub async fn generate_keypair() -> anyhow::Result<()> {
    let (address, ed_key_pair): (SuiAddress, Ed25519KeyPair) = get_key_pair();
    let d = ed_key_pair.copy();
    let sui_key_pair = SuiKeyPair::Ed25519(ed_key_pair);
    println!("Generated address: {:?}", address);
    println!("Private key: {:?}", d.private());
    println!("Sui key pair: {:?}", sui_key_pair);

    Ok(())
}

pub async fn get_balance(addr: &str) -> anyhow::Result<()> {
    // Connect to Sui testnet
    let client = SuiClientBuilder::default().build_testnet().await?;
    let addr = SuiAddress::from_str(addr)?;

    // Fetch balance
    let usdc_type =
        "0xa1ec7fc00a6f40db9693ad1415d0c193ad3906494428cf252621037bd7117e29::usdc::USDC";

    // note to self, the coin balance is in MIST
    // 1 SUI = 10^9 MIST
    let coins = client
        .coin_read_api()
        .get_coins(addr, Some(usdc_type.parse()?), None, Some(2))
        // .get_coins(addr, None, None, None)
        .await?;
    println!("Coins ===>: {:?}", coins);

    // NOTE:, this would give you all coins, so you can get the coin type
    let all_coins = client
        .coin_read_api()
        .get_all_coins(addr, None, None)
        .await?;

    for coin in all_coins.data {
        println!("Coin type: {}", coin.coin_type);
    }
    println!("Coin balance: {:?}", coins);
    Ok(())
}

struct TokenTransfer {
    from: SuiAddress,
    to: SuiAddress,
    amount: u64,
}

pub async fn transfer_token(token_transfer_args: TokenTransfer) -> anyhow::Result<()> {
    let sender = token_transfer_args.from;
    let recipient = token_transfer_args.to;

    let client = SuiClientBuilder::default().build_testnet().await?;
    // STEP 1: Get a gas coin owned by sender
    let coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_object = coins.data.first().expect("No gas coins found");

    // transfer_object(&self, signer: SuiAddress, object_id: ObjectID, gas: Option<ObjectID>, gas_budget: u64, recipient: SuiAddress)

    // get coin. Move this out of here.
    let sending_token = client
        .coin_read_api()
        .get_coins(sender, None, None, Some(2))
        // .get_coins(addr, Some("0x2::usdc::USDC".to_string()), None, None)
        .await?;

    let tx_data = client
        .transaction_builder()
        .transfer_object(
            sender,
            sending_token.data[0].coin_object_id,
            Some(gas_object.coin_object_id),
            // Some(sending_token.data[1].coin_object_id),
            1_000_000,
            recipient,
        )
        .await?;

    // client.

    // .with_gas(gas_object.coin_object_id)
    // .build(&sui_keypair)?;

    Ok(())
}

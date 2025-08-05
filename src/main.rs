use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
};
use alloy_sol_types::sol;
use dotenvy::dotenv;
use eyre::Result;
use futures_util::stream::StreamExt;
use std::env;

sol! {
    event Swap(
        address indexed sender,
        address indexed recipient,
        int256 amount0,
        int256 amount1,
        uint160 sqrtPriceX96,
        uint128 liquidity,
        int24 tick
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set");
    let rpc_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let contract_address = address!("4e68Ccd3E89f51C3074ca5072bbAC773960dFa36");
    let filter = Filter::new()
        .address(contract_address)
        .event("Swap(address,address,int256,int256,uint160,uint128,int24)")
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        let Swap {
            sender,
            recipient,
            amount0,
            amount1,
            sqrtPriceX96,
            liquidity,
            tick,
        } = log.log_decode()?.inner.data;

        println!("📥 Swap from {sender:?} to {recipient:?}");
        println!("   amount0: {amount0}");
        println!("   amount1: {amount1}");
        println!("   sqrtPriceX96: {sqrtPriceX96}");
        println!("   liquidity: {liquidity}");
        println!("   tick: {tick}");
    }

    Ok(())
}

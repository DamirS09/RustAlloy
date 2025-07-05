use alloy::{
    network::TransactionBuilder,
    primitives::{
        U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let signer: PrivateKeySigner =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?; //connect wallet with private key
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect("http://127.0.0.1:8545")
        .await?; // connect to blockchain with wallet
    let sabina = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8"); //connect sabina address
    let value = Unit::ETHER.wei().saturating_mul(U256::from(100)); // get 100 in wei, saturating_mul check overflow otherwise return U256::max (default for fn)
    let tx = TransactionRequest::default()
        .with_to(sabina)
        .with_value(value); //prepare tx

    // let pending_tx = provider.send_transaction(tx).await?;
    // println!("Pending transaction... {}", pending_tx.tx_hash());
    // let receipt = pending_tx.get_receipt().await?;

    //println!("Transaction included in block {}", receipt.block_number.expect("Failed to get block number"));
    println!("Transferred {:.5} ETH to {sabina}", format_ether(value));
    let balance_sabina = provider.get_balance(sabina).await?;
    println!("Balance in Wei: {}", balance_sabina);
    Ok(())
}

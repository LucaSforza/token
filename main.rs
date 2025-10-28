extern crate alloy;
extern crate auction;
extern crate eyre;
use std::{process::exit, str::FromStr};

use alloy::{
    network::EthereumWallet as Wallet,
    primitives::Address,
    providers::ProviderBuilder,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use auction::auction::Auction::AuctionInstance;
use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Subcommand, Clone, Debug)]
enum Command {
    Winner,
    Placebid { value: alloy::primitives::U256 },
    Endauction,
}

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long)]
    auction: String,
    #[arg(short, long)]
    private_key: String,
    #[arg(short, long)]
    eth_address: String,
    #[arg(short, long)]
    rpc_address: String,
}

fn get_info(args: &Args) -> Result<(Wallet, Url, Address)> {
    let my_address: Address = args.eth_address.as_str().parse()?;
    let contract_addr: Address = args.auction.as_str().parse()?;
    let u: Url = Url::from_str(args.rpc_address.as_str())?;
    let pk = hex::decode(args.private_key.clone())?;

    let sign_key = SigningKey::from_slice(pk.as_slice())?;

    let signer = PrivateKeySigner::new_with_credential(sign_key, my_address, None);
    let wallet = Wallet::new(signer);

    Ok((wallet, u, contract_addr))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let (w, u, addr) = get_info(&args)?;

    let prov = ProviderBuilder::new().wallet(w).connect_http(u);
    let auc = AuctionInstance::new(addr, prov);

    match args.command {
        Command::Winner => {
            let addr = auc.winner().call().await?;
            println!("Winner: {}", addr);
        }
        Command::Placebid { value } => {
            let result = auc.placeBid(value).call().await?;
            if !result {
                eprintln!("Placing the bid failed");
                exit(1);
            }
            println!("The bid was posted");
        }
        Command::Endauction => {
            let result = auc.endAuction().call().await?;
            if !result {
                eprintln!("The auction could not be ended");
                exit(1);
            }
            println!("The auction is Finished");
        }
    };
    Ok(())
}

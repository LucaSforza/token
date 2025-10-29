extern crate alloy;
extern crate auction;
extern crate eyre;
use std::{str::FromStr, time::Duration};

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
    Token,
    Nft,
    Create,
}

// TODO: aggiungere descrizioni
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
    #[arg(short, long)] // TODO: mettere rcp di test di defautl
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

    // TODO (lunghino da implementare): aggiungere la possibilità
    // di approvare la transazione di token verso il contratto e anche NFT
    // aggiungere quindi due comandi:
    // approveToken e approveNFT che approvano transazioni che hanno come 'spender' l'auction.
    match args.command {
        Command::Winner => {
            let addr = auc.winner().call().await?;
            // TODO: aggiungere il bid massimo
            println!("Winner: {}", addr);
        }
        Command::Placebid { value } => {
            let result = auc.placeBid(value).send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{:?}", recepit);
        }
        Command::Endauction => {
            let result = auc.endAuction().send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{:?}", recepit);
        }
        Command::Token => {
            let result = auc.getTokenAddress().call().await?;
            println!("Token address: {}", result);
        }
        Command::Nft => {
            let result = auc.getNft().call().await?;
            println!(
                "Collection address: {}\nToken id: {}",
                result.result, result.token_id
            );
        }
        Command::Create => todo!(), // TODO: implementare
    };
    Ok(())
}

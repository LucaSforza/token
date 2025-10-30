extern crate alloy;
extern crate auction;
extern crate eyre;
use std::{process::exit, str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet as Wallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use auction::auction::Auction::AuctionInstance;
use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum Command {
    Winner {
        #[arg(short, long)]
        auction: String,
    },
    Placebid {
        #[arg(short, long)]
        auction: String,
        value: alloy::primitives::U256,
    },
    Endauction {
        #[arg(short, long)]
        auction: String,
    },
    Token {
        #[arg(short, long)]
        auction: String,
    },
    Nft {
        #[arg(short, long)]
        auction: String,
    },
    Bestbid {
        #[arg(short, long)]
        auction: String,
    },
    Create {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        nft_collection: String,
        #[arg(short, long)]
        id_token: u64, // TODO: in realta U256...
                       // ma non ci preoccupiamo perché tanto abbiamo 1.000.000 token su SapiCoin
                       // e u64 è sufficiente per i nostri scopi
    },
}

// TODO: aggiungere descrizioni
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long)]
    private_key: String,
    #[arg(short, long)]
    eth_address: String,
    #[arg(
        short,
        long,
        // TODO: Sistemare questa schifezza
        default_value_t = ["https://ethereum-sepolia-rpc.publicnode.com".to_string()][0].clone(),
    )]
    rpc_address: String,
}

fn get_info(args: &Args) -> Result<(Wallet, Url)> {
    let my_address: Address = args.eth_address.as_str().parse()?;
    let u: Url = Url::from_str(args.rpc_address.as_str())?;
    let pk = hex::decode(args.private_key.clone())?;

    let sign_key = SigningKey::from_slice(pk.as_slice())?;

    let signer = PrivateKeySigner::new_with_credential(sign_key, my_address, None);
    let wallet = Wallet::new(signer);

    Ok((wallet, u))
}

fn get_addres(x: String) -> Result<Address> {
    return Ok(x.as_str().parse()?);
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let (w, u) = get_info(&args)?;

    let prov = ProviderBuilder::new().wallet(w).connect_http(u);
    let auc;
    // TODO (lunghino da implementare): aggiungere la possibilità
    // di approvare la transazione di token verso il contratto e anche NFT
    // aggiungere quindi due comandi:
    // approveToken e approveNFT che approvano transazioni che hanno come 'spender' l'auction.
    match args.command {
        Command::Winner { auction } => {
            let auc = AuctionInstance::new(get_addres(auction)?, prov);
            let result = auc.winner().call().await?;
            println!("Winner: {}", result); // TODO: implt fmt::Display for TransactionReceipt
        }
        Command::Placebid { auction, value } => {
            let auc = AuctionInstance::new(get_addres(auction)?, prov);
            let result = auc.placeBid(value).send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{:?}", recepit); // TODO: implt fmt::Display for TransactionReceipt
        }
        Command::Endauction { auction } => {
            let auc = AuctionInstance::new(get_addres(auction)?, prov);
            let result = auc.endAuction().send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{:?}", recepit); // TODO: implt fmt::Display for TransactionReceipt
        }
        Command::Token { auction } => {
            let auc = AuctionInstance::new(get_addres(auction)?, prov);
            let result = auc.currency().call().await?;
            println!("Winner: {}", result); // TODO: implt fmt::Display for TransactionReceipt
        }
        Command::Nft { auction } => {
            let auc = AuctionInstance::new(get_addres(auction)?, prov);
            let result = auc.getNft().call().await?; // TODO: cambia getNft() con toSold() getNft è DEPRECATA
            println!("NFT Collection: {}", result.result); // TODO: implt fmt::Display for TransactionReceipt
            println!("Token Id: {}", result.token_id); // TODO: implt fmt::Display for TransactionReceipt
        }
        Command::Bestbid { auction } => {
            let auc = AuctionInstance::new(get_addres(auction)?, prov);
            let result = auc.topBid().call().await?;
            println!("Best Bidder: {}", result.user); // TODO: implt fmt::Display for TransactionReceipt
            println!("Tokens placed: {}", result.value); // TODO: implt fmt::Display for TransactionReceipt
        }
        Command::Create {
            token,
            nft_collection,
            id_token,
        } => {
            let id_token: U256 = U256::from(id_token);
            let builder = AuctionInstance::deploy_builder(
                prov,
                get_addres(token)?,
                get_addres(nft_collection)?,
                id_token,
            );
            let result = builder.send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{:?}", recepit);
        }
    }
    Ok(())
}

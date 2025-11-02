extern crate alloy;
extern crate auction;
extern crate eyre;
use core::fmt;
use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet as Wallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    rpc::types::TransactionReceipt,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use auction::{
    auction::Auction::AuctionInstance, erc20::ERC20::ERC20Instance, erc721::ERC721::ERC721Instance,
};
use clap::{Parser, Subcommand};
use eyre::Result;
struct DisplayableTransactionReceipt(TransactionReceipt);
impl fmt::Display for DisplayableTransactionReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gas_total_price = self.0.gas_used * self.0.effective_gas_price as u64;
        writeln!(f, "TxID:{:?},", self.0.transaction_hash)?;
        writeln!(f, "Fee:{}", gas_total_price)?;
        writeln!(f, "Gas_used:{}", self.0.gas_used)?;
        writeln!(f, "Gas_price:{}", self.0.effective_gas_price)?;
        if let Some(contract) = self.0.contract_address {
            writeln!(f, "Contract created address: {}", contract)?;
        }
        for log in self.0.logs() {
            writeln!(f, "log: {:?}", log)?;
        }
        Ok(())
    }
}
#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
enum Command {
    /// Displays the address of the auction winner.
    Winner {
        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,
    },

    /// Places a bid in an active auction.
    Placebid {
        /// Ethereum address of the bidder.
        #[arg(short, long)]
        eth_address: String,
        /// Private key of the bidder.
        #[arg(short, long)]
        private_key: String,

        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,

        /// Bid amount in tokens.
        #[arg(short, long)]
        value: u64,
    },

    /// Ends an active auction (callable only by the auction creator).
    Endauction {
        /// Ethereum address of the auction owner.
        #[arg(short, long)]
        eth_address: String,
        /// Private key of the auction owner.
        #[arg(short, long)]
        private_key: String,

        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,
    },

    /// Displays the ERC20 token used in the auction.
    Token {
        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,
    },

    /// Displays the NFT collection associated with the auction.
    Nft {
        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,
    },

    /// Shows the current best bid of the auction.
    Bestbid {
        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,
    },

    /// Creates a new NFT auction contract.
    Create {
        /// Ethereum address of the creator.
        #[arg(short, long)]
        eth_address: String,
        /// Private key of the creator.
        #[arg(short, long)]
        private_key: String,

        /// Address of the ERC20 token used for bidding.
        #[arg(short, long)]
        token: String,

        /// Address of the NFT collection.
        #[arg(short, long)]
        nft_collection: String,

        /// Token ID of the NFT to auction.
        #[arg(short, long)]
        id_token: u64,
    },

    /// Grants approval for the auction contract to spend ERC20 tokens.
    AllowTokenTransaction {
        /// Ethereum address of the token owner.
        #[arg(short, long)]
        eth_address: String,
        /// Private key of the token owner.
        #[arg(short, long)]
        private_key: String,

        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,

        /// Address of the ERC20 token contract.
        #[arg(short, long)]
        token: String,

        /// Maximum allowance value in tokens.
        #[arg(short, long)]
        value: u64,
    },

    /// Grants approval for the auction contract to transfer an NFT.
    AllowNftTrasaction {
        /// Ethereum address of the NFT owner.
        #[arg(short, long)]
        eth_address: String,
        /// Private key of the NFT owner.
        #[arg(short, long)]
        private_key: String,

        /// Address of the auction smart contract.
        #[arg(short, long)]
        auction: String,

        /// Address of the NFT collection.
        #[arg(short, long)]
        collection: String,

        /// Token ID of the NFT to authorize for transfer.
        #[arg(short, long)]
        id: u64,
    },
}

/// Main CLI configuration.
#[derive(Parser, Debug)]
#[command(
    name = "auction_controller",
    author = "Luca Sforza <sforza.2050030@studenti.uniroma1.it>, Roberto Di Rosa",
    version,
    about = "auction_controller is a command-line tool to create and manage NFT auctions on the Ethereum blockchain. \
It allows users to deploy new auctions, place bids, view winners, and handle ERC20/NFT approvals or transfers directly on-chain. \
All operations are transparent and verifiable through Ethereum RPC providers.\n\n\
Developed by Luca Sforza and Roberto Di Rosa. Licensed under GPL v3.0.",
    long_about = None
)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// RPC endpoint used to interact with the Ethereum network.
    #[arg(
        short,
        long,
        default_value_t = String::from("https://ethereum-sepolia-rpc.publicnode.com")
    )]
    rpc_address: String,
}

fn create_wallet(private_key: String, eth_address: String) -> Result<Wallet> {
    let my_address: Address = eth_address.as_str().parse()?;
    let pk = hex::decode(private_key.clone())?;

    let sign_key = SigningKey::from_slice(pk.as_slice())?;

    let signer = PrivateKeySigner::new_with_credential(sign_key, my_address, None);
    Ok(Wallet::new(signer))
}

fn get_address(x: String) -> Result<Address> {
    return Ok(x.as_str().parse()?);
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let u: Url = Url::from_str(args.rpc_address.as_str())?;

    match args.command {
        Command::Winner { auction } => {
            let prov = ProviderBuilder::new().connect_http(u);
            let auc = AuctionInstance::new(get_address(auction)?, prov);
            let result = auc.winner().call().await?;
            println!("Winner: {}", result);
        }
        Command::Placebid {
            auction,
            value,
            private_key,
            eth_address,
        } => {
            let w = create_wallet(private_key, eth_address)?;
            let prov = ProviderBuilder::new().wallet(w).connect_http(u);
            let auc = AuctionInstance::new(get_address(auction)?, prov);
            let result = auc.placeBid(U256::from(value)).send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{}", DisplayableTransactionReceipt(recepit));
        }
        Command::Endauction {
            auction,
            private_key,
            eth_address,
        } => {
            let w = create_wallet(private_key, eth_address)?;
            let prov = ProviderBuilder::new().wallet(w).connect_http(u);
            let auc = AuctionInstance::new(get_address(auction)?, prov);
            let result = auc.endAuction().send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{}", DisplayableTransactionReceipt(recepit));
        }
        Command::Token { auction } => {
            let prov = ProviderBuilder::new().connect_http(u);
            let auc = AuctionInstance::new(get_address(auction)?, prov);
            let result = auc.currency().call().await?;
            println!("Token Address: {}", result);
        }
        Command::Nft { auction } => {
            let prov = ProviderBuilder::new().connect_http(u);
            let auc = AuctionInstance::new(get_address(auction)?, prov);
            let result = auc.toSold().call().await?;
            println!("NFT Collection: {}", result.collection);
            println!("Token Id: {}", result.id);
        }
        Command::Bestbid { auction } => {
            let prov = ProviderBuilder::new().connect_http(u);
            let auc = AuctionInstance::new(get_address(auction)?, prov);
            let result = auc.topBid().call().await?;
            println!("Best Bidder: {}", result.user);
            println!("Tokens placed: {}", result.value);
        }
        Command::Create {
            token,
            nft_collection,
            id_token,
            private_key,
            eth_address,
        } => {
            let w = create_wallet(private_key, eth_address)?;
            let prov = ProviderBuilder::new().wallet(w).connect_http(u);
            let id_token: U256 = U256::from(id_token);
            let builder = AuctionInstance::deploy_builder(
                prov,
                get_address(token)?,
                get_address(nft_collection)?,
                id_token,
            );
            let result = builder.send().await?;
            let recepit = result
                .with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{}", DisplayableTransactionReceipt(recepit));
        }
        Command::AllowTokenTransaction {
            token,
            value,
            auction,
            private_key,
            eth_address,
        } => {
            let w = create_wallet(private_key, eth_address)?;
            let prov = ProviderBuilder::new().wallet(w).connect_http(u);
            let contract = ERC20Instance::new(get_address(token)?, prov);
            let result = contract
                .approve(get_address(auction)?, U256::from(value))
                .send()
                .await?;
            let recepit = result
                //.with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{}", DisplayableTransactionReceipt(recepit));
        }
        Command::AllowNftTrasaction {
            collection,
            id,
            auction,
            private_key,
            eth_address,
        } => {
            let w = create_wallet(private_key, eth_address)?;
            let prov = ProviderBuilder::new().wallet(w).connect_http(u);
            let contract = ERC721Instance::new(get_address(collection)?, prov);
            let result = contract
                .approve(get_address(auction)?, U256::from(id))
                .send()
                .await?;
            let recepit = result
                //.with_required_confirmations(1)
                .with_timeout(Some(Duration::from_secs(60)))
                .get_receipt()
                .await?;
            println!("{}", DisplayableTransactionReceipt(recepit));
        }
    }
    Ok(())
}

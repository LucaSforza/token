# Token

<!--toc:start-->

- [Token](#token)
  - [Description](#description)
  - [Installation](#installation)
    - [Docker](#docker)
    - [Brew](#brew)
    - [Manual](#manual)
  - [Usage](#usage)
    - [Docker](#docker)
    - [Anything else](#anything-else)
  <!--toc:end-->

## Description

_Cli_ tool for our blockchain course

## Installation

> [!NOTE]
> we recommend [docker](https://www.docker.com/products/docker-desktop/) for the best experience,
> [podman](https://podman-desktop.io/downloads/) is also supported.

### Docker

```bash
  docker pull docker.io/ar33d/auction_controller:base
```

### Brew

```bash
 brew tap LucaSforza/auction-controller &&  brew install lucasforza/auction-controller/auction_controller
```

### Manual

> [!CAUTION]
> install build dependencies on your distribution foundry,rust,gcc,g++

```bash
#Install foundry
curl -L https://foundry.paradigm.xyz | bash & & \
forge bind --crate-name auction  && \
# build  the binary
cargo build --release
```

## Usage

### Docker

```bash
 docker run --rm auction_controller:base -h
```

### Anything else

```bash
auction_controller -h
```

```bash
 # this should be the result when you run the command
auction_controller is a command-line tool to create and manage NFT auctions on the Ethereum blockchain.
 It allows users to deploy new auctions, place bids, view winners, and handle ERC20/NFT approvals or tr
ansfers directly on-chain. All operations are transparent and verifiable through Ethereum RPC providers
.

Developed by Luca Sforza and Roberto Di Rosa. Licensed under GPL v3.0.

Usage: auction_controller [OPTIONS] <COMMAND>

Commands:
  winner                   Displays the address of the auction winner
  placebid                 Places a bid in an active auction
  endauction               Ends an active auction (callable only by the auction creator)
  token                    Displays the ERC20 token used in the auction
  nft                      Displays the NFT collection associated with the auction
  bestbid                  Shows the current best bid of the auction
  create                   Creates a new NFT auction contract
  allow-token-transaction  Grants approval for the auction contract to spend ERC20 tokens
  allow-nft-trasaction     Grants approval for the auction contract to transfer an NFT
  help                     Print this message or the help of the given subcommand(s)

Options:
  -r, --rpc-address <RPC_ADDRESS>  RPC endpoint used to interact with the Ethereum network [default: ht
tps://ethereum-sepolia-rpc.publicnode.com]
  -h, --help                       Print help
  -V, --version                    Print version
```

## How reproduce the results

We use :

- <https://book.getfoundry.sh/>

Install foundry and then run:

```bash
./deployToken.sh
```

For deploying the token.

We didn't insert this in auction_controller because this tool is only for controlling auctions.

```bash
./deployNFT.sh
```

for deploying the NFT collection

```bash
./mintNFT.sh
```

for minting a NFT of a collection.

```bash
./setURI.sh
```

for setting an URI for a NFT.

### Auctions

Firts of all create an auction:

```bash
auction_controller create -e $(cat address.txt) -t 0x9DbA38F577aD9354bA322Db25AA1504917507eF5 -n 
$(cat nft_collection_address) -i $(cat token_id) -p $(cat secret.key)
```

Place a bid:

Before placing a bid you have to approve the trasaction to the Auction.

The Auction will transfer your tokens to itself. If you will not win the Auction you will be refound by the Auction.

```bash
cargo run -- allow-token-transaction  -e $(cat address.txt) -t $(cat token_address.txt) -v 100 -p $(cat secret.key) -a $(cat auction_address.txt)
```

Then you can place a bid:

```bash
auction_controller placebid  -e $(cat address.txt) -p $(cat secret.key) -a $(cat auction_address.txt) -v 100
```

Ending an auction is quite simple. If you are the owner then you can end the auction when you prefer.

But before you need to allow the NFT transaction.

```bash
auction_controller allow-nft-trasaction  -e $(cat address.txt) -n $(cat nft_collection.txt) -i $(cat token_id.txt) -p $(cat secret.key) -a $(cat auction_address.txt)
```

Then you can end the auction:
```bash
auction_controller endauction -e $(cat address.txt) -p $(cat secret.key) -a $(cat auction_address.txt)
```

### Liquidity Pool

For deploying the liquidity pool:


```bash
./deployToken.sh <private key> <token address>
```

```bash
./allowToken.sh <token address> <private key>  <liquidity address> <token amount>
```

```bash
./initPool.sh <contract address> <private key> <eth amount> <token amount>
```

eth value must end with ether or wei. (example: 0.5ether or 100000wei).

Add liquidity:

```bash
./allowToken.sh <token address> <private key>  <liquidity address> <token amount>
```

```bash
./addLiquidity.sh <contract address> <private key> <eth amount> <token amount>
```

Remove liquidity:

```bash
./getShare.sh <contract address> <address>
```

This will get the amount of share


```bash
./removeLiquidity.sh <contract address> <private key> <share>
```

Swap eth with SapiCoin

contract=$1
private_key=$2
value=$3

```bash
./swapEthForToken.sh <contract address> <private key> <eth amount>
```

eth value must end with ether or wei. (example: 0.5ether or 100000wei).

```bash
./swapTokenForEth.sh <contract address> <private key> <token amount>
```

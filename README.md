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

Install Homebrew: 

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

```bash
 brew tap LucaSforza/auction-controller &&  brew install auction_controller
```

### Manual

> [!CAUTION]
> install build dependencies on your distribution foundry,rust,gcc,g++

```bash
#Install foundry
curl -L https://foundry.paradigm.xyz | bash & & \
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

> ![CAUTION]
> Moltiplicare i token da inserire moltiplicato per 10^18. Questo perché il token ammette 18 decimali.
> Quindi per avere un token ammount di 2 il valore da inserire sarà 2*10^18

```bash
./deployToken.sh <private key> <token amount>
```

For deploying the token.

We didn't insert this in auction_controller because this tool is only for controlling auctions.

for deploying the NFT collection:

```bash
./deployNFT.sh <private key> <collection name> <collection symbol>
```

for minting a NFT of a collection.

```bash
./mintNFT.sh  <private key> <collection address> <address receiver> <token id>
```

for setting an URI for a NFT.


```bash
./setURI.sh TODO per ora non abbiamo implementato questa funzionalità
```

### Auctions

Firts of all create an auction:

```bash
auction_controller create -e $(cat address.txt) -t (cat token_address.txt) -n 
$(cat nft_collection_address) -i $(cat token_id) -p $(cat secret.key)
```

Place a bid:

Before placing a bid you have to approve the trasaction to the Auction.

The Auction will transfer your tokens to itself. If you will not win the Auction you will be refound by the Auction.

```bash
auction_controller allow-token-transaction  -e $(cat address.txt) -t $(cat token_address.txt) -v 100 -p $(cat secret.key) -a $(cat auction_address.txt)
```

Then you can place a bid:

> ![CAUTION]
> Moltiplicare i token da inserire moltiplicato per 10^18. Questo perché il token ammette 18 decimali.
> Quindi per avere un token ammount di 2 il valore da inserire sarà 2*10^18

```bash
auction_controller placebid  -e $(cat address.txt) -p $(cat secret.key) -a $(cat auction_address.txt) -v $(echo "100*10^18" | bc)
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
./deployLiquidityPool.sh <private key> <token address>
```

> ![CAUTION]
> Moltiplicare i token da inserire moltiplicato per 10^18. Questo perché il token ammette 18 decimali.
> Quindi per avere un token ammount di 2 il valore da inserire sarà 2*10^18

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

This will get the amount of share in LP:

```bash
./getShareLP.sh <contract address> <address>
```
This will get the amount of share in %:

```bash
./getShare.sh <contract address> <address>
```

Remove liquidity:


```bash
./removeLiquidity.sh <contract address> <private key> <LP share to remove>
```

Swap eth with SapiCoin

```bash
./swapEthForToken.sh <contract address> <private key> <eth amount>
```

eth value must end with ether or wei. (example: 0.5ether or 100000wei).

```bash
./allowToken.sh <token address> <private key>  <liquidity address> <token amount>
```

```bash
./swapTokenForEth.sh <contract address> <private key> <token amount>
```

For visualize the exchange rate you can view it [here](https://sepolia.etherscan.io/address/0xac40156910ec9e821874cf112b41f5f4464fe277). The exchange rate is #eth/#SapiCoin is the price in etherium of 1 SapiCoin. #SapiCoin/#eth is the price in SapiCoin of 1 eth.
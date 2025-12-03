# How reproduce the results

Before starting, you need to install [foundry](https://getfoundry.sh/).

Once the installation process is complete, the next step is to clone the repository containing all the smart contracts used in this project:

```bash
$ git clone https://github.com/LucaSforza/token.git
```

Inside the `src` directory you will find the contracts, in particular `src/sapicoin.sol` for the token and `src/mynft.sol` for the NFT collection.

To verify that everything works correctly, compile the Solidity code with:

```bash
$ forge build
```

Before deploying the smart contract you need to have some ether. We used 2 faucet:

- polygon:  [https://faucet.polygon.technology/](https://faucet.polygon.technology/)
- mining: [https://sepolia-faucet.pk910.de/](https://sepolia-faucet.pk910.de/)

Then you can modify the vanities of the Token by changing the code in the contructor: [src/sapicoin.sol](src/sapicoin.sol).

Then you can deploy a smart contract:

```bash
$ forge create src/sapicoin.sol:SapiCoin  -rpc-url https://ethereum-sepolia-rpc.publicnode.com \
    -private-key $(cat path/to/pk.txt) \
    -broadcast -constructor-args $@ # constructor arguments, the total amount of tokens
```

To obtain the private key, you can create a wallet with [MetaMask](https://metamask.io/) and retrieve it directly from the application.

To see the structure and options of the create subcommand, use:

```bash
$ forge create -h
```
To interact with the contract on the test blockchain, you must verify the contract source code using:
```bash
$ forge verify-contract -rpc-url https://ethereum-sepolia-rpc.publicnode.com 
    -etherscan-api-key $(cat path/to/api_key.txt)
    $(echo $ADDRESS) # contract address
    src/sapicoin.sol:SapiCoin
```

To obtain an Etherscan API key, create an account [here](https://etherscan.io/) and generate the key from your profile.

At this point, you will be able to view your smart contract on [Etherscan Sepolia](https://sepolia.etherscan.io/) :)
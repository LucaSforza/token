#!/bin/sh

private_key=$1

cast estimate --rpc-url https://api.metadium.com/dev --private-key $private_key --create src/test.sol:SapiCoin
#!/bin/sh


private_key=$1

shift

forge create src/sapicoin.sol:SapiCoin  --rpc-url https://ethereum-sepolia-rpc.publicnode.com --private-key $private_key  --broadcast --constructor-args $@ 
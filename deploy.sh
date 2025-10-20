#!/bin/sh

rpc=$1

private_key=$2

shift
shift

forge create src/test.sol:SapiCoin  --rpc-url $rpc --private-key $private_key  --broadcast --constructor-args $@ 
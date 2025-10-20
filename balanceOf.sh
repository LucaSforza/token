#!/bin/sh

rpc=$1
contract_address=$2
private_key=$3
public_key=$4

cast call $contract_address "balanceOf(address)(uint256)" --rpc-url $rpc --private-key $private_key $public_key
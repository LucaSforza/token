#!/bin/sh

contract_address=$1
private_key=$2
public_key=$3
value=$4

cast send $contract_address "transfer(address,uint256)(bool)" --rpc-url http://127.0.0.1:8545 --private-key $private_key $public_key $value

private_key=$1
contract=$2
address=$3
token_id=$4

cast send $contract "safeMint(address,uint256)" --rpc-url https://ethereum-sepolia-rpc.publicnode.com --private-key $private_key $address $token_id

contract=$1
private_key=$2
shift
shift

cast send $contract "removeLiquidity(uint256)" --rpc-url https://ethereum-sepolia-rpc.publicnode.com --private-key $private_key $@
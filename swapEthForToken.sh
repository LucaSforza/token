
contract=$1
private_key=$2
value=$3
shift
shift
shift

cast send $contract "swapEthForToken()" --value $value --rpc-url https://ethereum-sepolia-rpc.publicnode.com --private-key $private_key $@
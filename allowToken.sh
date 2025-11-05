
contract=$1
private_key=$2
shift
shift

cast send $contract "approve(address,uint256)(bool)" --rpc-url https://ethereum-sepolia-rpc.publicnode.com --private-key $private_key $@
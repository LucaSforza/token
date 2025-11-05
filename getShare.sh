
contract=$1
address=$2

cast call $contract "getLPBalance(address)(uint256)" --rpc-url https://ethereum-sepolia-rpc.publicnode.com $address 
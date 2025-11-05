
contract=$1
address=$2

echo "$(./getShareLP.sh $contract $address | gawk '{print $1}')/$(./getTotalLiquidity.sh $contract | gawk '{print $1}')" | bc -l
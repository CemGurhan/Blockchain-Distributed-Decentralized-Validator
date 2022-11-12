rm -rf example
mkdir example

while getopts "n:h:p:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    h) validator_host="$OPTARG" ;;
    p) peer_hosts+=("$OPTARG") ;;
    esac
done

exonum-ML generate-template \
example/common.toml \
--validators-count ${number_of_validators:-1}

exonum-ML generate-config \
  example/common.toml example/1 \
  --peer-address ${validator_host:-"127.0.0.1"}:6331 -n

cd example

for i in $(seq 0 $((${number_of_validators:-1} - 1))); do 
    mkdir $((i+1))
    cd $((i+1))
    touch pub.toml
    cd ..
done

cd ..

source ./scripts/utils/newTab.sh
openTab sh "sh $PWD/run_reciever_daemon.sh $PWD"

echo "The whole list of values is '${peer_hosts[@]}'"

if [[ $number_of_validators != 1 ]]
then
    for val in "${peer_hosts[@]}"; do
        echo "HI"
    done
fi





# exonum-ML finalize \
#   --public-api-address 0.0.0.0:9000 \
#   --private-api-address 0.0.0.0:9001 \
#   example/1/sec.toml example/1/node.toml \
#   --public-configs example/1/pub.toml example/2/pub.toml

# cargo install --path .
# cd ./tx_validator
# npm install && babel src -d dist
# cd ..





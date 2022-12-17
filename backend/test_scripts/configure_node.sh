rm -rf example
mkdir example

number_of_validators=1
validator_host="0.0.0.0"
sync="BAP"
scoring_flag=1
modelName="MNIST28X28"
validator_port=6332
validator_reciever_port=6335

while getopts "n:h:v:r:p:e:s:f:m:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    h) validator_host="$OPTARG" ;;
    v) validator_port=$(($OPTARG)) ;;
    r) validator_reciever_port=$(($OPTARG)) ;;
    p) peer_hosts+=("$OPTARG") ;;
    e) peer_reciever_ports+=("$OPTARG") ;;
    s) sync=    "$OPTARG" ;;
    f) scoring_flag= $(($OPTARG)) ;;
    m) modelName= "$OPTARG" ;;
    esac
done

if [[ $sync != "BAP" ]]
then
    ttab -w sh test_scripts/syncer_run.sh $duration
fi

if [[ ${#peer_reciever_ports[@]} -ne ${#peer_hosts[@]} ]]
then
    for i in $(seq 0 $((${#peer_hosts[@]} - 1))); do
        peer_reciever_ports[$i]=6335
    done
fi

exonum-ML generate-template \
example/common.toml \
--validators-count $number_of_validators

exonum-ML generate-config \
  example/common.toml example/1 \
  --peer-address $validator_host:$validator_port -n

cd example

for i in $(seq 0 $(($number_of_validators - 1))); do 
    mkdir $((i+1))
    cd $((i+1))
    touch pub.toml
    cd ..
done

cd ..

ttab -w sh test_scripts/run_reciever_daemon.sh $validator_reciever_port
sleep 2

echo "All peer hosts are: '${peer_hosts[@]}'"


if [[ $number_of_validators != 1 ]]
then
    for i in "${!peer_hosts[@]}"; do # set to for i in $(seq 0 $((${#peer_hosts[@]} - 1))); do
        pub_key_response_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" "${peer_hosts[$i]}":"${peer_reciever_ports[$i]}"/getPubKey)"
        pub_key_response="$(curl --connect-timeout 5 "${peer_hosts[$i]}":"${peer_reciever_ports[$i]}"/getPubKey)"

            while [[ pub_key_response_header -eq 000  ]] # set to -ne 200 
            do
                pub_key_response_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" "${peer_hosts[$i]}":"${peer_reciever_ports[$i]}"/getPubKey)"
                pub_key_response="$(curl --connect-timeout 5 "${peer_hosts[$i]}":"${peer_reciever_ports[$i]}"/getPubKey)"
            done

        echo "Ok"

        cd example
        cd $((i+2))
        echo "$pub_key_response" >> pub.toml

    done
    cd ..
else
    cd example
fi

node_list=($(seq 1 $number_of_validators))

node_list=("${node_list[@]/%//pub.toml}")


exonum-ML finalize \
  --public-api-address 0.0.0.0:9000 \
  --private-api-address 0.0.0.0:9001 \
  1/sec.toml 1/node.toml \
  --public-configs "${node_list[@]}"

echo "${node_list[@]}"

cd ..
cargo install --path .
cd ./tx_validator
npm install && babel src -d dist
cd ..
bash run_node.sh  0 $sync $scoring_flag 1 $modelName 






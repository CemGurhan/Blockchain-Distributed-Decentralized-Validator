rm -rf example
mkdir example

validator_peer_address="0.0.0.0:6332"
number_of_validators=1
sync="BAP"
scoring_flag=1
modelName="MNIST28X28"
validator_pubkey_reciever_service_port=6335
validator_public_port=9000
validator_private_port=9001
is_non_iid=0
data_reciever_service_port=8080

while getopts "n:p:o:t:r:e:a:s:f:m:d:v:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    p) validator_peer_address="$OPTARG" ;;
    o) validator_public_port=$(($OPTARG)) ;;
    t) validator_private_port=$(($OPTARG)) ;;
    r) validator_pubkey_reciever_service_port=$(($OPTARG)) ;;
    a) peer_pubkey_reciever_service_addresses+=("$OPTARG");;
    s) sync="$OPTARG" ;;
    f) scoring_flag=$(($OPTARG)) ;;
    m) modelName="$OPTARG" ;;
    d) is_non_iid=$(($OPTARG)) ;;
    v) data_reciever_service_port=$(($OPTARG)) ;;
    esac
done

if [[ $sync != "BAP" ]]
then
    ttab -w sh test_scripts/syncer_run.sh $duration
fi

if [[ is_non_iid -ne 0 ]]
then
    rm -rf test_data
    mkdir test_data
    cd test_data

    for i in $(seq 0 $(($number_of_validators - 1))); do
        touch data$i.csv
    done

    cd ..
    cd ./test_data_io/lightclient_numbers
    rm -f light_clients_on_network.txt
    touch light_clients_on_network.txt

    cd ..
    cd data_reciever
    ttab -w cargo run --release $data_reciever_service_port

    echo "verifying all lightclient data has been sent to current validator"
    cd ../..
    sleep 20 # add a way to wait for all lightclients to have sent their data before running reconstruct py without using a sleep timer
    while ! [ -s ./test_data_io/lightclient_numbers/light_clients_on_network.txt ]
    do
        sleep 1 # sleep to account for potential delay in file write in data reciever service
    done
    lightclients_on_network=$(<./test_data_io/lightclient_numbers/light_clients_on_network.txt)
    IFS=','
    lightclients_on_network_array=($lightclients_on_network) 

    # curl the reciever to check if all LC data has been input from each lightclient, then continue. 
    for ((i=0;i<${#lightclients_on_network_array[@]};i++))
    do
        echo "calling data reciever service to verify lightclient $((i+1))'s test data was sent successfully"
        data_fill_check_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" 0.0.0.0:$data_reciever_service_port/dataFilledConfirm/${lightclients_on_network_array[i]})"
        while [[ data_fill_check_header -eq 500 ]] || [[ data_fill_check_header -eq 000 ]] # change to check if eq to 200
        do
            data_fill_check_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" 0.0.0.0:$data_reciever_service_port/dataFilledConfirm/${lightclients_on_network_array[i]})"
        done
    done

    python reconstruct_test_set.py
    rm -f tx_validator/src/models/MNIST28X28/data.csv
    mv test_data.csv tx_validator/src/models/MNIST28X28/data.csv
fi

unset IFS

exonum-ML generate-template \
example/common.toml \
--validators-count $number_of_validators

exonum-ML generate-config \
  example/common.toml example/1 \
  --peer-address $validator_peer_address -n

cd example

for i in $(seq 0 $(($number_of_validators - 1))); do 
    mkdir $((i+1))
    cd $((i+1))
    touch pub.toml
    cd ..
done

cd ..

ttab -w sh test_scripts/run_reciever_daemon.sh $validator_pubkey_reciever_service_port
sleep 2

echo "All peer addresses are: '${peer_pubkey_reciever_service_addresses[@]}'"

# set pub_key_response_header to be 000 here to get rid of two while loops


if [[ $number_of_validators != 1 ]]
then
    for i in $(seq 0 $((${#peer_pubkey_reciever_service_addresses[@]} - 1))); do 
        pub_key_response_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" "${peer_pubkey_reciever_service_addresses[$i]}"/getPubKey)"
        pub_key_response="$(curl --connect-timeout 5 "${peer_pubkey_reciever_service_addresses[$i]}"/getPubKey)"

            while [[ pub_key_response_header -ne 200  ]] 
            do
                pub_key_response_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" "${peer_pubkey_reciever_service_addresses[$i]}"/getPubKey)"
                pub_key_response="$(curl --connect-timeout 5 "${peer_pubkey_reciever_service_addresses[$i]}"/getPubKey)"
            done

        echo "Ok"

        cd example
        cd $((i+2))
        echo "$pub_key_response" >> pub.toml
        cd ..
    done
else
    cd example
fi


node_list=($(seq 1 $number_of_validators))

node_list=("${node_list[@]/%//pub.toml}")


exonum-ML finalize \
  --public-api-address 0.0.0.0:$validator_public_port \
  --private-api-address 0.0.0.0:$validator_private_port \
  1/sec.toml 1/node.toml \
  --public-configs "${node_list[@]}"

echo "node public key store contains: ${node_list[@]}"

cd ..
cargo install --path .
cd ./tx_validator
npm install && babel src -d dist
cd ..
ttab -w sh test_scripts/run_node.sh  0 $sync $scoring_flag 1 $modelName $validator_public_port






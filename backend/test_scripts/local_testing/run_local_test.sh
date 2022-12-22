number_of_validators=1
is_non_iid=0

while getopts "n:t:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    t) is_non_iid=$(($OPTARG)) ;;
    esac
done

sh test_scripts/local_testing/validator_copy.sh $number_of_validators

validator_peer_port=6332
validator_public_port=9000
validator_private_port=9001
validator_pubkey_reciever_service_port=$(($validator_peer_port+$number_of_validators))
peer_pubkey_reciever_service_flag=""
current_pubkey_reciever_service_port=$(($validator_peer_port+$number_of_validators))

# create reciever address flag, and run configure_node.sh, for first validator 
for i in $(seq 0 $(($number_of_validators - 2))); do
        peer_pubkey_reciever_service_flag+="-a 0.0.0.0:"
        validator_pubkey_reciever_service_port=$(($validator_pubkey_reciever_service_port+$((i+1))))
        peer_pubkey_reciever_service_flag+=$validator_pubkey_reciever_service_port
        peer_pubkey_reciever_service_flag+=" "
        validator_pubkey_reciever_service_port=$(($validator_pubkey_reciever_service_port-$((i+1))))
done
validator_pubkey_reciever_service_port=$(($validator_peer_port+$number_of_validators))
# change current reciever port to next validators reciever port 
current_pubkey_reciever_service_port=$(($validator_pubkey_reciever_service_port+1))
echo $peer_pubkey_reciever_service_flag
if [[ is_non_iid -ne 0 ]]
then
    ttab -w eval "sh test_scripts/configure_node.sh -n $number_of_validators -p 0.0.0.0:$validator_peer_port -o $validator_public_port -t $validator_private_port -r $validator_pubkey_reciever_service_port $peer_pubkey_reciever_service_flag -d $is_non_iid"
else
    ttab -w eval "sh test_scripts/configure_node.sh -n $number_of_validators -p 0.0.0.0:$validator_peer_port -o $validator_public_port -t $validator_private_port -r $validator_pubkey_reciever_service_port $peer_pubkey_reciever_service_flag"
fi

cd ..

for i in $(seq 1 $(($number_of_validators - 1))); do
    cd backend$i
    # clear the reciever flags for the new validator
    peer_pubkey_reciever_service_flag=""
    # previously we skipped adding the first validators peer port to the reciever flags. As we are no longer on the first validator, we add the first validators reciever port back into the reciever flag for the next validator
    peer_pubkey_reciever_service_flag+="-a 0.0.0.0:"
    peer_pubkey_reciever_service_flag+=$validator_pubkey_reciever_service_port
    peer_pubkey_reciever_service_flag+=" "
    validator_pubkey_reciever_service_port=$(($validator_pubkey_reciever_service_port+1))
    for j in $(seq 0 $(($number_of_validators-2))); do
            # do not add reciever port to peer reciever flags if the reciever port (validator_pubkey_reciever_service_port) is the same as the current validators reciever port (current_pubkey_reciever_service_port)
            if [[ $validator_pubkey_reciever_service_port -eq $current_pubkey_reciever_service_port ]]
            then
                validator_pubkey_reciever_service_port=$(($validator_pubkey_reciever_service_port+1))
                continue
            fi
            peer_pubkey_reciever_service_flag+="-a 0.0.0.0:"
            peer_pubkey_reciever_service_flag+=$validator_pubkey_reciever_service_port
            peer_pubkey_reciever_service_flag+=" "
            validator_pubkey_reciever_service_port=$(($validator_pubkey_reciever_service_port+1))
    done
    echo $peer_pubkey_reciever_service_flag
    validator_public_port=$(($validator_public_port+$((i+1))))
    validator_private_port=$(($validator_private_port+$((i+1))))
    if [[ is_non_iid -ne 0 ]]
    then
        ttab -w eval "sh test_scripts/configure_node.sh -n $number_of_validators -p 0.0.0.0:$(($validator_peer_port+$i)) -o $validator_public_port -t $validator_private_port -r $current_pubkey_reciever_service_port $peer_pubkey_reciever_service_flag -d $is_non_iid"
    else
        ttab -w eval "sh test_scripts/configure_node.sh -n $number_of_validators -p 0.0.0.0:$(($validator_peer_port+$i)) -o $validator_public_port -t $validator_private_port -r $current_pubkey_reciever_service_port $peer_pubkey_reciever_service_flag"
    fi
    validator_pubkey_reciever_service_port=$(($validator_peer_port+$number_of_validators))
    current_pubkey_reciever_service_port=$(($current_pubkey_reciever_service_port+1))
    cd ..
done


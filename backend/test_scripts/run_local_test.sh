number_of_validators=$1

sh test_scripts/validator_copy.sh $number_of_validators

validator_peer_port=6332
validator_public_port=9000
validator_private_port=9001
validator_reciever_port=$(($validator_peer_port+$number_of_validators))
peer_reciever_flag=""
current_reciever_port=$(($validator_peer_port+$number_of_validators))

# create reciever address flag and run configure_node.sh for first validator 
for i in $(seq 0 $(($number_of_validators - 2))); do
        peer_reciever_flag+="-a 0.0.0.0:"
        validator_reciever_port=$(($validator_reciever_port+$((i+1))))
        peer_reciever_flag+=$validator_reciever_port
        peer_reciever_flag+=" "
        validator_reciever_port=$(($validator_reciever_port-$((i+1))))
done
validator_reciever_port=$(($validator_peer_port+$number_of_validators))
current_reciever_port=$(($validator_reciever_port+1))
echo $peer_reciever_flag
ttab -w eval "sh test_scripts/configure_node.sh -n $number_of_validators -p 0.0.0.0:$validator_peer_port -o $validator_public_port -t $validator_private_port -r $validator_reciever_port $peer_reciever_flag"

cd ..

for i in $(seq 1 $(($number_of_validators - 1))); do
    cd backend$i
    peer_reciever_flag=""
    peer_reciever_flag+="-a 0.0.0.0:"
    peer_reciever_flag+=$validator_reciever_port
    peer_reciever_flag+=" "
    validator_reciever_port=$(($validator_reciever_port+1))
    for i in $(seq 0 $(($number_of_validators-2))); do
            if [[ $validator_reciever_port -eq $current_reciever_port ]]
            then
                validator_reciever_port=$(($validator_reciever_port+1))
                continue
            fi
            peer_reciever_flag+="-a 0.0.0.0:"
            peer_reciever_flag+=$validator_reciever_port
            peer_reciever_flag+=" "
            validator_reciever_port=$(($validator_reciever_port+1))
    done
    echo $peer_reciever_flag
    validator_peer_port=$(($validator_peer_port+1))
    validator_public_port=$(($validator_public_port+2))
    validator_private_port=$(($validator_private_port+2))
    ttab -w eval "sh test_scripts/configure_node.sh -n $number_of_validators -p 0.0.0.0:$validator_peer_port -o $validator_public_port -t $validator_private_port -r $current_reciever_port $peer_reciever_flag"
    validator_reciever_port=$(($validator_peer_port+$number_of_validators))
    current_reciever_port=$(($current_reciever_port+1))
    cd ..
done


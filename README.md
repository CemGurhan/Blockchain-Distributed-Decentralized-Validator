# Repo for Distributed De-Centralized (DD) IID/Non-IID training tests


To run, execute the following script:

```
sh test_scripts/configure_node.sh 
```

The following flags may also be supplied:

```
-n <number_of_validators_on_network (defaults to 1)> 
-h <your_host (defaults to "0.0.0.0")> 
-v <your_peer_port (defaults to 6332)> 
-r <your_pubkey_reciever_port (defaults to 6335)> 
-p <peer_hosts> 
-e <peer_reciever_ports> 
-s <sync_policy (defaults to "BAP")> 
-f <scoring_flag (defaults to 1)> 
-m <model_name (defauls to "MNIST28X28")>
```

When testing locally, run the following script to copy the backend folder into the relevant number of validators:

```
sh test_scripts/validator_copy.sh <number_of_validators on the network>
```

## Example workflow over a network

To run validators over a network run the following command on one machine:

## Example workflow testing locally with 2 validators on the network

For ease of local testing, run the following script to copy the backend folder into the relevant number of validators:

```
sh test_scripts/validator_copy.sh 2
```

To then run validators locally for testing, cd into the `backend` folder and run the following command:

```
sh test_scripts/configure_node.sh -n 2 -h 0.0.0.0 -v 6332 -r 6336 -p 0.0.0.0 -e 6335 
```

This will run a reciever service for this validator on port `6336`, as we have supplied this port value using the flag `r`. It will also specify the validators peer address to be `0.0.0.0:6332`, as we have set the validators peer port to be `6332` using the `v` flag, and set the validators host address to be `0.0.0.0` using the `h` flag. 
The validator will now be looking out for public keys at the address `0.0.0.0:6335`, as we have set the peer hosts to be `0.0.0.0` using the argument in flag `p`, and have set the peers reciever service port to be `6335` using the argument in flag `e`.

Now cd into the `backend1` folder and run the command:

```
sh test_scripts/configure_node.sh -n 2 -h 0.0.0.0 -v 6333 -r 6335 -p 0.0.0.0 -e 6336
```

This will run a reciever service for this validator on port `6335`. It will also specify the validators peer address to be `0.0.0.0:6333`. 
The validator will now be looking out for public keys at the address `0.0.0.0:6336`.
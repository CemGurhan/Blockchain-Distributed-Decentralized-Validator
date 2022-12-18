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
-o <validator_port (public port the validator runs on - defaults to 9000)>
-t <validator_private_port (private port the validator runs on - defaults to 9001)>
-r <your_pubkey_reciever_port (defaults to 6335)> 
-a <peer_reciever_addresses (addresses that peer reciever services are running on)>
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

## Example workflow testing locally with 2 validators (Non-Distributed De-Centralized test - NDD)

For ease of local testing, run the following script to copy the backend folder into the relevant number of validators:

```
sh test_scripts/validator_copy.sh 2
```

To then run validators locally for testing, cd into the `backend` folder and run the following command:

```
sh test_scripts/configure_node.sh -n 2 -h 0.0.0.0 -v 6332 -o 9000 -t 9001 -r 6336 -a 0.0.0.0:6335 
```

This will run a reciever service for this validator on port `6336`, as we have supplied this port value using the flag `r`. It will also specify the validators peer address to be `0.0.0.0:6332`, as we have set the validators peer port to be `6332` using the `v` flag, and set the validators host address to be `0.0.0.0` using the `h` flag. 
The validator will now be looking out for public keys at the address `0.0.0.0:6335`, as we have set the peer reciever address to be this using the `a` flag.

Now cd into the `backend1` folder and run the command:

```
sh test_scripts/configure_node.sh -n 2 -h 0.0.0.0 -v 6333 -o 9002 -t 9003 -r 6335 -a 0.0.0.0:6336
```

This will run a reciever service for this validator on port `6335`. It will also specify the validators peer address to be `0.0.0.0:6333`. 
The validator will now be looking out for public keys at the address `0.0.0.0:6336`.

## Example workflow testing locally with 4 validators (Non-Distributed De-Centralized test - NDD)

Using the same concepts as the previous example, to run 4 vaidators for local testing, first begin by copying the backend folder with the following command: 

```
sh test_scripts/validator_copy.sh 4
```

Next, inside the `backend` folder, execute the following command:

```
sh test_scripts/configure_node.sh -n 4 -h 0.0.0.0 -v 6332 -o 9000 -t 9001 -r 6336 -p 0.0.0.0 -p 0.0.0.0 -p 0.0.0.0 -e 6337 -e 6338 -e 6339
```

Inside the `backend1` folder execute:

```
sh test_scripts/configure_node.sh -n 4 -h 0.0.0.0 -v 6333 -o 9002 -t 9003 -r 6337 -p 0.0.0.0 -p 0.0.0.0 -p 0.0.0.0 -e 6336 -e 6338 -e 6339
```

Inside the `backend2` folder execute:

```
sh test_scripts/configure_node.sh -n 4 -h 0.0.0.0 -v 6334 -o 9004 -t 9005 -r 6338 -p 0.0.0.0 -p 0.0.0.0 -p 0.0.0.0 -e 6336 -e 6337 -e 6339
```

Inside the `backend3` folder execute:

```
sh test_scripts/configure_node.sh -n 4 -h 0.0.0.0 -v 6335 -o 9006 -t 9007 -r 6339 -p 0.0.0.0 -p 0.0.0.0 -p 0.0.0.0 -e 6336 -e 6337 -e 6338
```
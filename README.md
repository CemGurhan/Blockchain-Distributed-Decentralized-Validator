To run, execute script:

```
sh test_scripts/configure_node.sh -n <number_of_validators_on_network> -h <your_host> -v <your_peer_port> -r <your_pubkey_reciever_port> -p <peer_hosts> -e <peer_reciever_ports> -s <sync_policy> -f <scoring_flag> -m <model_name>
```

An example command:

```
sh test_scripts/configure_node.sh -n 2 -h 0.0.0.0 -v 6332 -r 6336 -p 0.0.0.0  
```
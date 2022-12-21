# Repo for Distributed De-Centralized (DD) IID/Non-IID training tests

In order to run a validator, the following prerequisites must be installed on your machine:

* [Exonum specific dependecnies](https://exonum.com/doc/version/latest/get-started/install/):
&nbsp; <br> - RocksDB 
&nbsp; <br> - libsodium
&nbsp; <br>  - Protocol Buffers
* Rust compiler
* Python >= 3
* Tensorflow
* Keras
* Numpy
* Pandas
* [ttab](https://www.npmjs.com/package/ttab)
* npm
* Node
* Babel

You must also populate your validator with test data. If running with mnist data sets, test data must be stored in backend/tx-validator/src/models/MNIST28X28 or backend/tx-validator/src/models/MNIST20X20 (depending on your data). 


To run, execute the following script (if you are on a linux machine, remember to use `bash` as opposed to `sh` when running the script commands seen in this document):

```
sh test_scripts/configure_node.sh 
```

The following flags may also be supplied:

```
-n <number_of_validators_on_network (defaults to 1)> 
-p <validator_peer_address (defaults to "0.0.0.0:6332")>
-o <validator_port (public port the validator runs on - defaults to 9000)>
-t <validator_private_port (private port the validator runs on - defaults to 9001)>
-r <your_pubkey_reciever_port (defaults to 6335)> 
-a <peer_reciever_addresses (addresses that peer reciever services are running on)>
-s <sync_policy (defaults to "BAP")> 
-f <scoring_flag (defaults to 1)> 
-m <model_name (defauls to "MNIST28X28")>
```

When testing locally, run the following script in the backend folder to produce n number of validators:

```
sh test_scripts/run_local_test.sh <number_of_validators (number of validators on mock network for local testing)>  
```

For a more in-depth look at how the local test scripts work, see [here](#example-workflow-testing-locally-with-2-validators-non-distributed-de-centralized-test---ndd) for a set up example with 2 validators, or [here](#example-workflow-testing-locally-with-4-validators-non-distributed-de-centralized-test---ndd) for a set up example with 4 validators.

## Example workflow over a network with 2 validators

To run validators over a network, you must expose your machines ports to the internet from your LAN. This can be achieved via simple port forwarding, but it is advised not to go down this route. You can instead use a VPN, such as [Tailscale](https://tailscale.com/). In this example, we have opted to utilize tailscale, and have registered two different machines on their VPN. Hence, host addresses will be taken from the ones assigned to our machines via Tailscale. We also had one machine running on macOS, whilst the other was running on Ubuntu. The only differences in commands here will be the use of `bash` as opposed to `sh` in the Ubuntu machine.

Once you have two machines running agaisnt a network, and ready for incoming connections, enter this command inside the backend folder of your validator on your macOS machine:

```
sh test_scripts/configure_node.sh -n 2 -p 100.99.117.84:6332 -o 9000 -t 9001 -r 6335 -a 100.123.57.49:6336
```

This will run a reciever service for this validator on port `6335`, as we have supplied this port value using the flag `r`. It will also specify the validators peer address to be `100.99.117.84:6332`, as we have set the validators peer port to be `6332` using the `v` flag, and set the validators host address to be `100.99.117.84` using the `h` flag. For this host value, the ip address generated by Tailscale for this particular machine was used.
The validator will now be looking out for public keys at the address `100.123.57.49:6335`, as we have set the peer reciever address to be this using the `a` flag. The host value of `100.123.57.49` was taken from the ip address tailscale generated for the other machine we wish to run on the entwork. The validator will also be running on a public port of `9000`, which was specified by the `o` flag, and will have a private port of `9001`, specified by the `t` flag.

On another machine, enter this command inside your backend folder:

```
bash test_scripts/configure_node.sh -n 2 -p 100.123.57.49:6332 -o 9000 -t 9001 -r 6336 -a 100.99.117.84:6335
```


## Example workflow testing locally with 2 validators (Non-Distributed De-Centralized test - NDD)

For ease of local testing, run the following script to copy the backend folder into the relevant number of validators:

```
sh test_scripts/validator_copy.sh 2
```

To then run validators locally for testing, cd into the `backend` folder and run the following command:

```
sh test_scripts/configure_node.sh -n 2 -p 0.0.0.0:6332 -o 9000 -t 9001 -r 6336 -a 0.0.0.0:6335 
```

This will run a reciever service for this validator on port `6336`, as we have supplied this port value using the flag `r`. It will also specify the validators peer address to be `0.0.0.0:6332`, as we have set the validators peer port to be `6332` using the `v` flag, and set the validators host address to be `0.0.0.0` using the `h` flag. 
The validator will now be looking out for public keys at the address `0.0.0.0:6335`, as we have set the peer reciever address to be this using the `a` flag. The validator will also be running on a public port of `9000`, which was specified by the `o` flag, and will have a private port of `9001`, specified by the `t` flag.

Now cd into the `backend1` folder and run the command:

```
sh test_scripts/configure_node.sh -n 2 -p 0.0.0.0:6333 -o 9002 -t 9003 -r 6335 -a 0.0.0.0:6336
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
sh test_scripts/configure_node.sh -n 4 -p 0.0.0.0:6332 -o 9000 -t 9001 -r 6336 -a 0.0.0.0:6337 -a 0.0.0.0:6338 -a 0.0.0.0:6339
```

Inside the `backend1` folder execute:

```
sh test_scripts/configure_node.sh -n 4 -p 0.0.0.0:6333 -o 9002 -t 9003 -r 6337 -a 0.0.0.0:6336 -a 0.0.0.0:6338 -a 0.0.0.0:6339
```

Inside the `backend2` folder execute:

```
sh test_scripts/configure_node.sh -n 4 -p 0.0.0.0:6334 -o 9004 -t 9005 -r 6338 -a 0.0.0.0:6336 -a 0.0.0.0:6337 -a 0.0.0.0:6339
```

Inside the `backend3` folder execute:

```
sh test_scripts/configure_node.sh -n 4 -p 0.0.0.0:6335 -o 9006 -t 9007 -r 6339 -a 0.0.0.0:6336 -a 0.0.0.0:6337 -a 0.0.0.0:6338
```

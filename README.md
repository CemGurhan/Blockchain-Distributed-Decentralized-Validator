To run with 2 validators and two light clients:
build backend with: bash build_finalize.sh -n 2 -b -c -j
run validator 1: bash run_node.sh  0 BAP 1 1 MNIST28X28
split terminal and run validator 2: bash run_node.sh  1 BAP 1 1 MNIST28X28
run two different light clients with: npm start -- 9000 models/MNIST28X28/data.csv 0.1 MNIST28X28
if you put a log in handle_message() in exonum-node/src/basic.rs line 29, you can see the validators constantly talking to each other
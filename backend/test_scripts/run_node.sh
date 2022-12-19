# path2=""
# command_start="sh "
node_count=$3
start_public_port=$6

# cd example
# GITHUB TEST LINE

echo "Hello from run_node.sh"
i="$(($1))"
sync_policy=$2

scoring_flag=$4

model_name=$5

endS=$6

# if [ $endS -ne 0 ] # target version is endS
# then
#         currentT=0
#         tmp=$(tty) # tty = current terminal connected to standard input
#         if [[ "$tmp" == *"pts"* ]]  
#             then
#                 currentT=${tmp##*/}
#             else
#                 tmp=${str:L-3} 
#                 currentT=$((tmp+0))
#         fi
#     source .././scripts/utils/newTab.sh
#     openTab $command_start "$command_start $PWD/../scripts/track_plot/track.sh $endS $currentT $path2"
#     # $command_start ./scripts/track_plot/track.sh $endS $currentT
# fi
cd example

echo "i = $i"

echo "HERE WE ARE IN RUN $PWD"

public_port=$((start_public_port + i))
private_port=$((public_port + node_count))

echo "new node with ports: $public_port (public), hello from run_node.sh - we're running exonum-ML now"
exonum-ML run --node-config $((i + 1))/node.toml --db-path $((i + 1))/db --public-api-address 0.0.0.0:${public_port} --master-key-pass pass \
--sync-policy $sync_policy --scoring-flag $scoring_flag --model-name $model_name





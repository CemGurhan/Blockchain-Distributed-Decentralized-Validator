command_start=$1
i=$2
nodes=$3
sync=$4
period=$5
scoring_flag=$6
model_name=$7
PWD=$8
# if [[ $path != "./" ]]
# then 
#     mkdir $path
#     mkdir $path/backend
#     cp -v ./backend/run_node.sh $path/backend
#     cp -R -v ./backend/tx_validator $path/backend/tx_validator
#     cp -R -v ./backend/example $path/backend/example
# fi 

# if [[ $i == "0" ]]
# then 
#     source ./scripts/utils/newTab.sh
#     openTab $command_start "$command_start ./scripts/spawn/syncer_run.sh $path"
# fi
source $PWD/scripts/utils/newTab.sh
cd $PWD
echo "Hello from validator_run.sh! We are now running run_node.sh"
echo "model_name: $model_name,i: $i, sync: $sync, nodes: $nodes, scoring_flag: $scoring_flag"
$command_start "$PWD/test_scripts/run_test_node.sh" $i $sync $nodes $scoring_flag $model_name $PWD

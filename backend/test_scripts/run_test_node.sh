path2=""
endS=0
PWD=$6
while getopts "g:e:cbjlr" arg; do
    case $arg in
    g) 
        path2="$OPTARG"
        ;;
    e)
        endS=$(($OPTARG)) 
        ;;
    esac
done

node_count=$3
start_peer_port=7091
start_public_port=9000

cd $PWD/example
# iB | 52.00 KiB/s


echo "Hello from run_node.sh"
i="$(($1))"
sync_policy=$2

scoring_flag=$4

model_name=$5

echo "i = $i"

public_port=$((start_public_port + i))
private_port=$((public_port + node_count))

echo "new node with ports: $public_port (public), hello from run_node.sh - we're running exonum-ML now"
exonum-ML run --node-config $((i + 1))/node.toml --db-path $((i + 1))/db --public-api-address 0.0.0.0:${public_port} --master-key-pass pass \
--sync-policy $sync_policy --scoring-flag $scoring_flag --model-name $model_name



if [ $endS -ne 0 ] # target version is endS
then
    currentT=0
    if [ $willTerminate -ne 1 ] # No input to spawn script at -r flag
    then
        currentT=-1
    else
        tmp=$(tty) # tty = current terminal connected to standard input
        if [[ "$tmp" == *"pts"* ]]  
        then
            currentT=${tmp##*/}
        else
            tmp=${str:L-3} 
            currentT=$((tmp+0))
        fi
    fi
    openTab $command_start "$command_start ./scripts/track_plot/track.sh $endS $currentT $path2"
    # $command_start ./scripts/track_plot/track.sh $endS $currentT
fi

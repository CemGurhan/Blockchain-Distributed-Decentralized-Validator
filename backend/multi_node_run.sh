node_count=$1
for ((i=0;i<node_count;i++));
do
    echo "Staring validator #$i"
    source ./scripts/utils/newTab.sh
    openTab bash "bash ./scripts/spawn/validator_run.sh bash $i ./ $node_count BAP 60 1 MNIST28X28"
    sleep 10

done
path="./"
sync="BAP"
duration=60
scoring_flag=1
modelName="MNIST28X28"

while getopts "n:p:s:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    p) path=    "$OPTARG" ;;
    s) sync=    "$OPTARG" ;;
    d) duration= "$OPTARG" ;;
    s)scoring_flag= $(($OPTARG)) ;;
    m)modelName= "$OPTARG" ;;
    esac
done


bash build_finalize.sh -n $number_of_validators -b -c -j

for i in $(seq 0 $((${number_of_validators:-1} - 1))); do 
    source ./scripts/utils/newTab.sh
    openTab sh "sh ./scripts/spawn/validator_run.sh sh $i $path $number_of_validators $sync $duration $scoring_flag $modelName"
done


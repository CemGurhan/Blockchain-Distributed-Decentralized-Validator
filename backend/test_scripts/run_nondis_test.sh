sync="BAP"
duration=60
scoring_flag=1
modelName="MNIST28X28"
number_of_validators=1

while getopts "n:p:s:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    p) path=    "$OPTARG" ;;
    s) sync=    "$OPTARG" ;;
    d) duration= "$OPTARG" ;;
    f) scoring_flag= $(($OPTARG)) ;;
    m) modelName= "$OPTARG" ;;
    esac
done


bash ./build_finalize.sh -n $number_of_validators -b -c -j

source ./scripts/utils/newTab.sh
openTab eval "echo 'Welcome to the DataNET local tester. You are running $number_of_validators validators for local testing'"

for i in $(seq 0 $(($number_of_validators - 1))); do 
    source ./scripts/utils/newTab.sh
    openTab sh "sh $PWD/test_scripts/validator_test_run.sh sh $i $number_of_validators $sync $duration $scoring_flag $modelName $PWD"
done


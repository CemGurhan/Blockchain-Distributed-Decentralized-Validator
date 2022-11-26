sync="BAP"
duration=60
scoring_flag=1 
modelName="MNIST28X28"
number_of_validators=1
isMainTest=0

while getopts "n:p:s:d:f:m:t:" arg; do
    case $arg in
    n) number_of_validators=$(($OPTARG)) ;;
    p) path="$OPTARG" ;;
    s) sync="$OPTARG" ;;
    d) duration="$OPTARG" ;;
    f) scoring_flag=$(($OPTARG)) ;;
    m) modelName="$OPTARG" ;;
    t) isMainTest=$(($OPTARG)) ;;
    esac
done

if [[ isMainTest -ne 0 ]]
then
    rm -rf test_data
    mkdir test_data
    cd test_data
    for ((i=0;i<10;i++)); do
        touch data$i.csv 
    done
    cd ..
    cd ./test_data_io/data_reciever
    cargo build --release
    ttab -w cargo run --release
    cd ../..
    sleep 5
    # curl the reciever to check if all ten LC data has been input , then continue. DO the same in LC
    echo "calling data reciever service"
    data_fill_check_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" 0.0.0.0:8000/dataFilledConfirm)"
    while [[ data_fill_check_header -eq 500 ]] || [[ data_fill_check_header -eq 000 ]]
    do
        data_fill_check_header="$(curl --connect-timeout 5 -o /dev/null -s -w "%{http_code}\n" 0.0.0.0:8000/dataFilledConfirm)"
    done
    python reconstruct_test_set.py
fi


bash ./build_finalize.sh -n $number_of_validators -b -c -j

for i in $(seq 0 $(($number_of_validators - 1))); do 
    ttab -w sh test_scripts/validator_test_run.sh $i $number_of_validators $sync $duration $scoring_flag $modelName 
done
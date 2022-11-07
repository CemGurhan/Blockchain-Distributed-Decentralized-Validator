rm -rf example
mkdir example

while getopts "l:" arg; do
    case $arg in
    l) is_lead_validator=$(($OPTARG)) ;;
    esac
done

echo $is_lead_validator

if [ $is_lead_validator == 1 ]; then
    exonum-ML generate-template \
    example/common.toml \
    --validators-count 2 #2
fi


exonum-ML generate-config \
  example/common.toml example/1 \
  --peer-address 127.0.0.1:6331 -n

exonum-ML finalize \
  --public-api-address 0.0.0.0:9000 \
  --private-api-address 0.0.0.0:9001 \
  example/1/sec.toml example/1/node.toml \
  --public-configs example/1/pub.toml example/2/pub.toml

cargo install --path .
cd ./tx_validator
npm install && babel src -d dist
cd ..





number_of_validators=$1
rm -rf example
mkdir example
rm -rf target
rm -rf pub_key_io/reciever/target
cd ..

echo "now creating $(($number_of_validators-1)) more validator(s) for local testing"
for ((i=1;i<$number_of_validators;i++)); do 
    cp -r backend backend$i
done
echo "$number_of_validators validators now ready for local testing"


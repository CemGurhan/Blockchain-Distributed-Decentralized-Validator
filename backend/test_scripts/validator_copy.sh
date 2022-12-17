number_of_validators=$1
rm -rf example
mkdir example
rm -rf target
cd ..

echo "now creating $(($number_of_validators-1)) more backend folder(s) for local testing"
for ((i=1;i<$number_of_validators;i++)); do 
    cp -r backend backend$i
done
echo "finished creating backend folders, local testing ready"


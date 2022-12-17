sleep 5
period=$1

cd syncBarrier
npm install
npm start -- $period

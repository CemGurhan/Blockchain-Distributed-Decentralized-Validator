cur_path=$(pwd)
cd "${cur_path%FDMMLS*}/FDMMLS"
cd ./backend/

command_start="sh "
command="./build_finalize.sh "
path="./"
path2=""
endS=0
willTerminate=0
start_public_port=9000
start_peer_port=7091
sync="BAP"
accumulated_error_scale=0
scoring_flag=1
duration=60
modelName="MNIST28X28"
while getopts "n:t:p:g:w:q:e:s:d:f:a:m:cbjlr" arg; do
    case $arg in
    n) 
        node_count=$(($OPTARG)) 
        command+="-n "
        command+="$node_count "
        ;;
    c) 
        command+="-c "
        ;;
    b) 
        command+="-b "
        ;;
    j) 
        command+="-j "
        ;;
    l) 
        command_start="bash "
        ;;
    t) 
        trainers="$OPTARG"
        ;;
    p) 
        path="$OPTARG"
        ;;
    g) 
        path2="$OPTARG"
        ;;
    w) 
        start_public_port="$OPTARG" 
        command+="-w $start_public_port "
        ;;
    q)  
        start_peer_port="$OPTARG" 
        command+="-q $start_peer_port "
        ;;
    e)
        endS=$(($OPTARG)) 
        ;;
    r) 
        willTerminate=1
        ;;
    s)
        sync="$OPTARG" ;;
    d)
        duration="$OPTARG" ;;
    f)
        scoring_flag="$OPTARG" ;;
    a)
        accumulated_error_scale="$OPTARG" ;;
    m) 
        modelName="$OPTARG" ;;
    esac
done
printf "%0.s*" {1..70} 
printf "\n"
echo "Running: "$command_start$command
printf "%0.s*" {1..70} 
printf "\n"
$command_start$command
cd ..

printf "%0.s*" {1..70} 
printf "\n"
for ((i=0;i<node_count;i++));
do
    echo "Staring validator #$i , hello from spawn.sh! We're now running validator_run.sh"
    source ./scripts/utils/newTab.sh
    openTab $command_start "$command_start ./scripts/spawn/validator_run.sh $command_start $i $path $node_count $sync $duration $scoring_flag $modelName"
    sleep 10

done
if [[ $sync != "BAP" ]]
then
    openTab $command_start "$command_start ./scripts/spawn/syncer_run.sh $path $duration"
fi
printf "%0.s*" {1..70} 
printf "\n"

cd ./lightclient
rm ModelMetadata
rm encoded_vector
npm install
cd ..
###############################
## Copying lightclient folder
###############################
for ((i=0;i<trainers;i++))
do
    source ./scripts/utils/newTab.sh
    $command_start ./scripts/spawn/trainer_run.sh $i $path $command_start
done

###############################
## Running light client
###############################
for ((i=0;i<trainers;i++))
do
    source ./scripts/utils/newTab.sh
    if [[ $i != 0 ]]
    then
        lightclient="lightclient$i"
    else 
        lightclient="lightclient"
    fi
    assigned_trainer_port=$(($((start_public_port))+$(($((i))%$((node_count))))))
    echo $start_public_port
    trainer_noise=$(echo "$i * $accumulated_error_scale" | bc)
    rm $lightclient/ModelMetadata
    openTab $command_start "npm start --prefix $lightclient -- 9000 models/MNIST28X28/data.csv $trainer_noise $modelName"
    sleep 10
done

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

# Short for pseudoterminal, PTY is a pair of virtual devices that provide a bidirectional communication. 
# One end of the channel is called the master or ptm, and the other is the slave or pts. The pts provides 
# the pseudoterminal that acts exactly like a classic terminal and sends input data to the ptm. 
# Output data is sent back from the ptm to the pts.
# TTY ports are direct connections to the computer such as a keyboard/mouse or a serial connection to the device.
# PTS connections are SSH connections or telnet connections. 
# https://www.computerhope.com/jargon/p/pty.htm
# https://www.golinuxcloud.com/difference-between-pty-vs-tty-vs-pts-linux/#:~:text=In%20layman%27s%20terms%20the%20primary,SSH%20connections%20or%20telnet%20connections.

# if [[ "$tmp" == *"pts"* ]] tests whether the content of $tmp contains pts
# https://superuser.com/questions/1582334/bash-what-means-quotes-inside-asterisk-str

# currentT=${tmp##*/} attempts to remove every part of the contents of tmp up to and includng the /. Its a substring removal.
# https://stackoverflow.com/questions/2059794/what-is-the-meaning-of-the-0-syntax-with-variable-braces-and-hash-chara

# 




# sleep 10
# if [ $endS -ne 0 ]
# then
#     currentT=0
#     if [ $willTerminate -ne 1 ]
#     then
#         currentT=-1
#     else
#         tmp=$(tty)
#         currentT=${tmp##*/}
#     fi
#     openTab $command_start "$command_start ./scripts/track_plot/track.sh $endS $currentT"
# fi

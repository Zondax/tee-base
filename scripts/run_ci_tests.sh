#!/bin/bash

# This will kill all spawned processes
cleanup() {
    jobs=$(jobs -p)
    if [[ ! -z "$jobs" ]]; then
        kill $jobs &> /dev/null
    fi
    exit
}

# Kill all the spawned jobs if this one is killed
trap cleanup SIGINT SIGTERM EXIT

# run make in another process
(echo "Running tests"
make run) &

#This function will start netcat at the specified port 
# and retry if it fails to do so
# it will also save all output to the second argument specified
capture_output() {
    PORT=$1
    FILE=$(realpath $2)
    
    while ! nc localhost $PORT > $2; do
        sleep 1;
    done
}

# start listening to 54320 (REE application) 54321 (TEE application)
(capture_output 54320 normal_world.out) & 
(capture_output 54321 /dev/null) & 

wait

#echo "Normal World output:"
#cat normal_world.out

echo "DONE!" "Filtering into tests output"
grep -Pzo ".*TESTS STARTING(?s:.*)TESTS FINISHED" normal_world.out > tests.out

echo "------------- TESTS OUTPUT -----------------"
cat tests.out

echo -e "\n-------------- RUN SUMMARY -----------------"
success=$(grep -c "SUCCESS" tests.out)
failed=$(grep -c "FAILURE" tests.out)
echo "Successful tests: $success"
echo "Failed tests: $failed"
let "total = $success + $failed"
echo "Tests run: $total"

message=""
if [[ "$failed" -eq 0 ]]; then
    message="SUCCESSFUL"
    result=0
else
    message="FAILURE"
    result=1
fi
echo "------------- TESTS $message -------------"

exit "$result"

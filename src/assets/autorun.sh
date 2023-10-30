#! /bin/bash
if [ ! -d "results/$1" ]; then
    mkdir results/$1
fi

FILEXTENSION=".out"
# only way to use touch in for loop


for P in $1/*; do
    FILE=${P#*/}
    FILENAME=${FILE%.*}
    OUTPUTFILE="results/$1/$FILENAME$FILEXTENSION"
    if [ !  -f $OUTPUTFILE ]; then
        touch $OUTPUTFILE
    fi
    
    time ./boolean_functions.exe $P $P > $OUTPUTFILE
done

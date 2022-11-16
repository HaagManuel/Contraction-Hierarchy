#!/bin/bash

DIR="/algoDaten/praktikum/graph"
#GRAPHS="stupferich karlsruhe germany europe"
GRAPHS="stupferich karlsruhe germany"
#GRAPHS="stupferich karlsruhe"
PROG="cargo run --release --bin compute_distances -- "
METRIC="travel_time geo_distance"

for G in $GRAPHS 
do
    for M in $METRIC
    do
        CUR="${DIR}/${G}"
        SOURCE="${CUR}/test"

        CH_GRAPH="${DIR}/${G}/${M}_ch"
        CH_WEIGHT="${DIR}/${G}/${M}_ch/weight"
        ORDER="${CUR}/${M}_ch/order"
        WEIGHT="${CUR}/${M}"


        OUT="./outputs/a2/${G}/${M}_length"
        mkdir -p "./outputs/a2/${G}/"
        echo "$PROG -e 2 -g $CH_GRAPH -w $CH_WEIGHT -s $SOURCE -o $OUT" --ordering $ORDER

        OUT="./outputs/a3/${G}/${M}_length"
        mkdir -p "./outputs/a3/${G}/"
        echo "$PROG -e 3 -g $CUR -w $WEIGHT -s $SOURCE -o $OUT" --ordering $ORDER

        OUT="./outputs/a4/${G}/${M}_length"
        mkdir -p "./outputs/a4/${G}/"
        echo "$PROG -e 4 -g $CUR -w $WEIGHT -s $SOURCE -o $OUT"
    done
done

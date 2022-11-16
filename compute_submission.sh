#!/bin/bash

DIR="/algoDaten/praktikum/graph"
PROG="cargo run --release --bin compute_distances -- "
METRIC="travel_time geo_distance"

for M in $METRIC
do
    mkdir -p "./abgabe/aufgabe1"
    mkdir -p "./abgabe/aufgabe2"
    mkdir -p "./abgabe/aufgabe3"
    mkdir -p "./abgabe/aufgabe4"

    #exercise 1
    CUR="${DIR}/aufgabe1"
    SOURCE="${CUR}/test"
    WEIGHT="${CUR}/${M}"
    OUT="./abgabe/aufgabe1/${M}_length"
    echo "$PROG -e 1 -g $CUR -w $WEIGHT -s $SOURCE -o $OUT"
    
    #exercise 2
    CUR="${DIR}/aufgabe2"
    SOURCE="${CUR}/test"
    CH_GRAPH="${CUR}/${M}_ch"
    CH_WEIGHT="${CH_GRAPH}/weight"
    ORDER="${CH_GRAPH}/order"
    OUT="./abgabe/aufgabe2/${M}_length"
    echo "$PROG -e 2 -g $CH_GRAPH -w $CH_WEIGHT -s $SOURCE -o $OUT --ordering $ORDER"
    
    #exercise 3
    CUR="${DIR}/aufgabe3"
    SOURCE="${CUR}/test"
    WEIGHT="${CUR}/${M}"
    ORDER="${CUR}/${M}_ch/order"
    OUT="./abgabe/aufgabe3/${M}_length"
    echo "$PROG -e 3 -g $CUR -w $WEIGHT -s $SOURCE -o $OUT --ordering $ORDER"


    #exercise 4
    CUR="${DIR}/aufgabe4"
    SOURCE="${CUR}/test"
    WEIGHT="${CUR}/${M}"
    OUT="./abgabe/aufgabe4/${M}_length"
    echo "$PROG -e 4 -g $CUR -w $WEIGHT -s $SOURCE -o $OUT --lazy"
    
done

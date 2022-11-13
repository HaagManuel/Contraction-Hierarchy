#!/bin/bash

DIR="/algoDaten/praktikum/graph"
#GRAPHS="europe  germany  karlsruhe  stupferich"
#GRAPHS="stupferich karlsruhe germany"
GRAPHS="stupferich karlsruhe"
PROG="cargo run --bin compare_vector i32 "
METRIC="travel_time geo_distance"

for G in $GRAPHS 
do
    for M in $METRIC
    do
        CUR="${DIR}/${G}"
        TEST="${CUR}/test/${M}_length"

        OUT="./outputs/a3/${G}/${M}_length"
        $PROG $TEST $OUT

        OUT="./outputs/a4/${G}/${M}_length"
        $PROG $TEST $OUT
    done
done

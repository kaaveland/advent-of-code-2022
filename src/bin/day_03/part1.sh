#!/usr/bin/env bash

set -euf -o pipefail

function intersect() {
    len=$(( $(echo -n "$1" | gwc -L) / 2 ))
    part1=$(echo -n "$1" | cut -c -$len)
    part2=$(echo -n "$1" | cut -c $(( $len + 1))-)
    intersection=$(echo -n "$part1" | grep -o "[$part2]")
    intersection=${intersection:0:1}
    lower=$(echo -n $intersection | tr '[:upper:]' '[:lower:]')
    ordinal=$(printf "%d" "'$lower")
    if [[ $intersection != $lower ]]; then # was uppercase
        echo $(( 27 + $ordinal - 97))
    else
        echo $(( 1 + $ordinal - 97))
    fi
}

function part1() {
    while read line; do
        if ! [[ -z "$line" ]]; then
            intersect $line
        fi
    done
}

pbpaste | part1 | gpaste -sd+ | bc

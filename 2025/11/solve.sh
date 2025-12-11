#!/bin/sh

_() {
    for i in $(<input grep $1: | cut -d\  -f2-); do
        [ $i = "out" ] && echo out || _ $i
    done
}

_ you | wc -l

# part 2

_() {
    for i in $(<input grep $1: | cut -d\  -f2-); do
        [ $i = "fft" ] && set 2 "$2,$i"
        [ $i = "dac" ] && set 2 "$2,$i"
        [ $i = "out" ] && echo $2 || _ $i "$2"
    done
}

_ svr | grep ',fft' | grep ',dac' | wc -l

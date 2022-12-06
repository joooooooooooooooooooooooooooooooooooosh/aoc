#!/bin/sh

for i in $(seq 4 $(wc -c ../input/06 | cut -d\  -f1)); do
    if [ $(sed -E 's/(.)/\1\n/g' ../input/06 | head -$i | tail -4 | sort | uniq | wc -l) -eq 4 ]; then
        echo $i
        exit
    fi
done

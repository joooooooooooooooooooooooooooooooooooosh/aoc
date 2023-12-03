#!/bin/sh

c=$(head -1 input)
{ cat input; echo; } | while read a; do
    i=1
    while [ $i -le $(wc -c <<< $c) ]; do
        num=$(<<< $c cut -c$i- | grep -o '^[0-9]*')
        l=$(wc -c <<< $num)
        if [ -n "$num" ]; then
            if <<< $b cut -c$((i-1 > 1 ? i-1 : 1))-$((i+l-1)) | grep -q '[^0-9.]' ||
               <<< $a cut -c$((i-1 > 1 ? i-1 : 1))-$((i+l-1)) | grep -q '[^0-9.]' ||
               <<< $c cut -c$((i-1 > 1 ? i-1 : 1))-$((i+l-1)) | grep -Eq '(^[^0-9.]|[^0-9.]$)'; then
                echo $num
            fi
            i=$((i+l))
            continue
        fi
        i=$((i+1))
    done

    b=$c
    c=$a
done | awk "{s+=\$1} END{print s}"

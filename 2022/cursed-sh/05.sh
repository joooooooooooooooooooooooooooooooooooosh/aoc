#!/bin/sh

sed '/^ 1/q' ../input/05 > stack.txt
for stack in $(tail -1 stack.txt); do
    stacks[$stack]=$(sed 's/    /  . /g; $d' stack.txt | awk "{print \$$stack}" | tr -d '\n[].' | tac)
done

while IFS=' ' read -r a b c; do
    for i in $(seq $a); do
        stacks[$c]="$(cut -c -1 <<< ${stacks[$b]})${stacks[$c]}"
        stacks[$b]=$(cut -c 2- <<< ${stacks[$b]})
    done
done <<< $(sed -En '/move/{s/move (.+) from (.) to (.)/\1 \2 \3/; p}' ../input/05)

tr ' ' '\n' <<< ${stacks[@]} | cut -c1 | tr -d '\n'

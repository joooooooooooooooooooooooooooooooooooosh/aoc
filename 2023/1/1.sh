#!/bin/sh

# part 1

< input tr -cd '0-9\n' | sed -E 's/^(.).*(.)$/\1\2/; s/^(.)$/\1\1/' | awk '{s+=$1} END{print s}'

# part 2

< input perl -pe 's/^.*?(one|two|three|four|five|six|seven|eight|nine|[0-9]).*(one|two|three|four|five|six|seven|eight|nine|[0-9]).*?$/\1\2/' | sed 's/one/1/g; s/two/2/g; s/three/3/g; s/four/4/g; s/five/5/g; s/six/6/g; s/seven/7/g; s/eight/8/g; s/nine/9/g' | tr -cd '0-9\n' | sed -E 's/^(.)$/\1\1/' | awk '{s+=$1} END{print s}'

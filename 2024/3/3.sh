#!/usr/bin/env sh

grep -Eo 'mul\([0-9]+,[0-9]+\)' input | sed 's/mul(\(.*\),\(.*\))/echo $((\1 * \2))/' | sh | awk '{s+=$1} END{print s}'

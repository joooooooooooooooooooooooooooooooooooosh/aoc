#!/usr/bin/env bash

sed 's/^/sed s_^-__ <<< $((/; s/$/))/' <(pr -m -t -s- <(sort -n <(cut -d\  -f1 input)) <(sort -n <(cut -d\  -f4 input))) | bash | awk '{s+=$1} END{print s}'

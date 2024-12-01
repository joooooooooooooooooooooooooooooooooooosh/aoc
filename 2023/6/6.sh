#!/bin/sh

< input paste <(head -1 input | tr -s ' ' \\n) <(tail -1 input | tr -s ' ' \\n) | sed -E '1s/.*/scale=5/; s/(.*)\t(.*)/sqrt(\1^2-4*\2)/' #| bc | sed '/\.0*$/s/$/ - 1/' | bc | awk '{printf("%d\n",$1 + 0.5)}' # | sed 's/$/ * \\/; $a1' | bc
# < input paste <(head -1 input | tr -s ' ' \\n) <(tail -1 input | tr -s ' ' \\n) | sed -E '1s/.*/scale=1/; s/(.*)\t(.*)/(\1 - sqrt(\1^2-4*\2))\/2/' # | bc | awk '{printf("%d\n",$1 + 0.5)}' | sed 's/$/ * \\/; $a1' | bc

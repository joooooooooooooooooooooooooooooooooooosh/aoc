#!/bin/bash
join -j 999999 -o 1.1,2.1 input input | sed -E 's/(.+),(.+) (.+),(.+)/(\1 -1 - \3) * (\2 -1 - \4)/' | bc | tr -d - | sort -rn | head -1

#!/bin/sh

# part 1
# < input sed 's/.*: /a=" /; s/ | / ";for i in /; s/$/;do grep " $i " <<< $a;done | wc -l | grep -v "^0$" | sed -E "s|(.*)|2^(\\1-1)|"/' | sh | bc | awk "{s+=\$1} END{print s}"

# part 2
< example sed 's/.*: /a=" /; s/ | / ";for i in /; s/$/;do grep " $i " <<< $a;done | wc -l | grep -v "^0$" | sed "s|^|1 |"/' | sh

exit
# | bc | awk "{s+=\$1} END{print s}"
1 4
1 2
1 2
1 1
1 0
1 0

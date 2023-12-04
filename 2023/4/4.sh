#!/bin/sh

< input sed 's/.*: /a=" /; s/ | / ";for i in /; s/$/;do grep " $i " <<< $a;done | wc -l | grep -v "^0$" | sed -E "s|(.*)|2^(\\1-1)|"/' | sh | bc | awk "{s+=\$1} END{print s}"

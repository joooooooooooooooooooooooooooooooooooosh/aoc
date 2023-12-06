#!/bin/sh

# warning: this takes ~5 hours to run on the real input
# if you just want to see it in action, run it on the example input
< input sed -E '1{s/.*:/for s in/; s/$/; do/};
2d;
/map/d; 2,$s/(.+) (.+) (.+)/o=$(paste <(seq \2 $((\2 + \3))) <(seq \1 $((\1 + \3))) | grep -P "^$s\\t" | cut -f2);[ -z "$o" ] \&\& \\/;
/^$/io=$s;s=$o;
$ao=$s;s=$o;echo $s; done' | sh | sort -n | head -1

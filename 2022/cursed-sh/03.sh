#!/bin/sh

awk '{print "grep -o "[" substr($0,1,length/2) "]" <<< " substr($0,length/2+1) " | head -1"}' < ../input/03 | sh | sed -E -e '1ibc <<< \' -e "s/(.)/$(printf "%d" \\'\1)/; s/([a-z]))$/\1-96/; s/([A-Z]))$/\1-38/; s/$/+\\/; $a0;" | sh

#!/bin/bash
{sed -E 's/forward (.*)/x=$((x+\1))/; s/down (.*)/y=$((y+\1))/; s/up (.*)/y=$((y-\1))/' input/02; echo "echo \$x \$y"} | sh | awk '{ print $1 * $2 }'

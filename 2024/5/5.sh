#!/usr/bin/env bash

# sed -E -f <(grep \| input | sed -E 's_(.*)\|(.*)_/(^|,)\2,(.*,)?\1(,|$)/d;_') <(grep , input) | sed -E -e ':l' -e 's/^[^,]+,(.*),[^,]+$/\1/;tl' | awk '{s+=$1} END{print s}'

# Part 2
# gsed for MacOS
# sed -E -f <(grep \| input | gsed -E -e 's_(.*)\|(.*)_s/(^|,)(\2),(.*,)?(\1)(,|$)/\\1\\4,\\3\\2\\5/;ts_; $ats\nd\n:s' | tee /dev/stdout | tac | sed 1,2d | tac) <(grep , input) | sed -E -e ':l' -e 's/^[^,]+,(.*),[^,]+$/\1/;tl' | awk '{s+=$1} END{print s}'
sed -E -f <(grep \| input | gsed -E -e 's_(.*)\|(.*)_s/(^|,)(\2),(.*,)?(\1)(,|$)/\\1\\4,\\3\\2\\5/;ts_; $ad\n:s' | tee /dev/stdout | tac | sed 1,2d | tac) <(grep , input) | sed -E -e ':l' -e 's/^[^,]+,(.*),[^,]+$/\1/;tl' | awk '{s+=$1} END{print s}'

#!/usr/bin/env bash

sed -E -f <(grep \| input | sed -E 's_(.*)\|(.*)_/(^|,)\2,(.*,)?\1(,|$)/d;_') <(grep , input) | sed -E -e ':l' -e 's/^[^,]+,(.*),[^,]+$/\1/;tl' | awk '{s+=$1} END{print s}'

#!/bin/bash
sed -En 'N; s/\n(.*)/ \1\n\1/; P; D;' input/01 | awk '{if ($1 < $2) print""}' | wc -l

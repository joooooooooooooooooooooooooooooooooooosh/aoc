#!/bin/bash

<input tr -- '-,' ' \n' | xargs -L1 seq -f '%.0f' | perl -ne 'print if /^(.+)(\1)$/' | awk '{s+=$1} END{print s}'

# part 2
<input tr -- '-,' ' \n' | xargs -L1 seq -f '%.0f' | perl -ne 'print if /^(.+)(\1)+$/' | awk '{s+=$1} END{print s}'

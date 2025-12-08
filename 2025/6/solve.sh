#!/bin/sh

# would work, but rs has a length limit of 2048

<input rs -T -C' ' | sed -E "s/(.*)(.) $/echo \1 | tr ' ' '\2'/" | sh | bc | awk '{s+=$1} END{print s}'

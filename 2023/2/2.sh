#!/bin/sh

# part 1
# < input grep -Ev "((1[3-9]|[2-9].) red|(1[4-9]|[2-9].) green|(1[5-9]|[2-9].) blue)" | sed -E 's/Game ([0-9]+): .*/\1/' | awk '{s+=$1} END{print s}'

# part 2
< input sed 's/Game .*:/echo "/; s/$/" | tr ",;" "\\n" | sort -rn | sort -sk2 | uniq -f1 | awk "BEGIN{s=1} {s*=\\$1} END{print s}"/' | sh | awk "{s+=\$1} END{print s}"

# less worse (but slower somehow) alternative
# < input xargs -I{} sh -c "echo '{}' | cut -d: -f2 | tr ',;' '\\n' | sort -rn | sort -sk2 | uniq -f1 | awk 'BEGIN{s=1} {s*=\$1} END{print s}'" | awk "{s+=\$1} END{print s}"

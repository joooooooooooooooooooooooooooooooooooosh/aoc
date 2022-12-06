#!/bin/sh

sed -e '1i{\nbc <<< 0+\\' -e 's/^$/0; bc <<< 0/; s/$/+\\/' -e '$a0;} | sort -rn | head -1' ../input/01 | sh
# sed -e '1i{\nbc <<< 0+\\' -e 's/^$/0; bc <<< 0/; s/$/+\\/' -e '$a0;} | sort -rn | head -3' ../input/01 | sh | awk "{s+=\$1} END{print s}"

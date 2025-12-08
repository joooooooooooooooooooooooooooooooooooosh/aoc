#!/bin/bash

(echo 'f(){' sed -E 's/(.+)-(.+)/[ \1 -le $1 ] \&\& [ \2 -ge $1 ] \&\& { echo; return; }/; s/^$/}/; /^[^-}]+$/s/^/f /' <input) | sh | wc -l

exit

# technically works
grep -cxf <(grep -v - input) <(<input tr - ' ' | sed '1,/^$/s/^/seq -f "%.0f" /' | sh | sort | uniq)

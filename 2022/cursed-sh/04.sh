#!/bin/sh

sed -E 's/(.+)-(.+),(.+)-(.+)/{ { [ \1 -le \3 ] \&\& [ \2 -ge \4 ]; } \|\| { [ \1 -ge \3 ] \&\& [ \2 -le \4 ]; } } \&\& echo/' ../input/04 | sh | wc -l

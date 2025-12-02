#!/bin/sh

(
    echo 'input=50'
    <input tr LR -+ | sed -E 's/^(.+)/input=$(((input \1 + 100) % 100))/'
) | sed 's/$/; [ $input -eq 0 ] \&\& echo/' | sh | wc -l

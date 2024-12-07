#!/usr/bin/env bash

_() {
    [ $# -eq 1 ] && {
        echo $1
        return
    }
    _ ${@:1:$(($# - 1))} | sed -E 's/([0-9]+)/$(('${@: -1}' * \1)) $(('${@: -1}' + \1))/g; s/^/echo /' | sh
}

source <(sed -E 's/(.*):/grep -Eo "(^| )\1( |$)" <(_/; s/$/) | head -1/' input) | awk '{s+=$1} END{print s}'

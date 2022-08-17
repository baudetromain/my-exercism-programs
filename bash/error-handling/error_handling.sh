#!/usr/bin/env bash

usage ()
{
    echo "Usage: error_handling.sh <person>"
}

main ()
{
    if [ $# -eq 1 ]
    then
        echo "Hello, $1"
    else
        usage
        exit 1
    fi
}

main "$@"
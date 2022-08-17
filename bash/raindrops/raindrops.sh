#!/usr/bin/env bash

usage ()
{
    echo "usage: ${0##*/} <number>"
}

main ()
{
    if [[ $# -eq 1 ]]
    then

        local output=""
        local number=$1

        if [[ $(($number % 3)) -eq 0 ]]
        then
            output="${output}Pling"
        fi

        if [[ $(($number % 5)) -eq 0 ]]
        then
            output="${output}Plang"
        fi

        if [[ $(($number % 7)) -eq 0 ]]
        then
            output="${output}Plong"
        fi

        if [[ -z $output ]]
        then
            echo $number
        else
            echo $output
        fi

    else
        usage
        exit 1
    fi
}

main "$@"
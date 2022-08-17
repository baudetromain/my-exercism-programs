#!/usr/bin/env bash

if [ $# -eq 0 ]
then
    NAME="you"
else
    NAME=$1
fi

echo "One for $NAME, one for me."

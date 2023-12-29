#!/usr/bin/env bash

STAGED_FILES=$(git diff --name-only --cached)

for FILE in $STAGED_FILES
do
    if grep -Iq "T""K" "$FILE"; then
        echo "ERROR: You have a T""K in $FILE, please remove it before committing"
        exit 1
    fi
done

#!/bin/sh

if [ $# != 1 ]; then
    echo "Usage: $(basename "$0") <day-number>" >&2
    exit 1
fi
if [ ! -d .git ]; then
    echo "must be run from root of advent-of-code repository" >&2
    exit 1
fi

name=$1
cargo new --bin "$name"
touch "$name/input.txt"
touch "$name/README.md"
echo "# $name Days of Advent" >> $name/README.md
echo "![fun $name](../images/$name.png)" >> $name/README.md

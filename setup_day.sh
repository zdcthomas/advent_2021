#!/bin/sh

if [ $# != 1 ]; then
    echo "Usage: $(basename "$0") <day-number, e.g 'one', 'two'>" >&2
    exit 1
fi
if [ ! -d .git ]; then
    echo "must be run from root of advent-of-code repository" >&2
    exit 1
fi

name=$1
touch "./input/$name"
mkdir "./src/$name"
touch "./src/$name/mod.rs"
touch "./src/$name/README.md"
echo "# $name Days of Advent" >> ./src/$name/README.md
echo "![fun $name](../../images/$name.png)" >> ./src/$name/README.md
echo "\n[day $name](./src/$name)" >> ./README.md
echo "mod $name;" >> ./src/main.rs

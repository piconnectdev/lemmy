#!/bin/sh
find . -name "up.sql" | sort | tr '\n' ' ' | xargs cat > list.txt


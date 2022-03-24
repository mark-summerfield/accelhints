#!/bin/bash
tokei -f -c80 -tPython,Rust -etarget
unrecognized.py -q
python3 -m flake8 --ignore=E261 .
python3 -m vulture . \
    | grep -v unused.*60%.confidence
grep --color=auto --exclude-dir=target --include=*.rs -r format......,
git st

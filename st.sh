#!/bin/bash
tokei -f -c80 -tRust -etarget
grep --color=auto --exclude-dir=target --include=*.rs -r format......,
git st

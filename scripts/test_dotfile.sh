#!/bin/bash

# ./scripts/test_dotfile.sh target/xflow

neato -Tpng -Gstart=rand $1.dot -o $1-neato-out.png
dot -Tpng -Gstart=rand $1.dot -o $1-dot-out.png
fdp -Tpng -Gstart=rand $1.dot -o $1-fdp-out.png
twopi -Tpng -Gstart=rand $1.dot -o $1-twopi-out.png
circo -Tpng -Gstart=rand $1.dot -o $1-circo-out.png

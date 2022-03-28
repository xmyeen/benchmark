#!/bin/bash

PYTHON_DEF="python3"
PROGRAM_NAME="bm_fileio"

$PYTHON_DEF -m venv py && \
py/bin/python -m pip install -U --force-reinstall --no-index --find-links=res/py $PROGRAM_NAME

mkdir -p rs
cp -rvf res/rs/* rs
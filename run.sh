#!/bin/bash
set -e

cargo b --release
cd gui
python3 main.py

#!/bin/bash
set -e

cargo b --release
cd gui
python main.py

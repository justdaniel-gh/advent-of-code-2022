#!/bin/env python3

from glob import glob
from os import system
from pathlib import Path


new_day = "day{}".format(sorted(int(d.replace("day", "")) for d in glob("day*")).pop() + 1)

system(f"cargo init {new_day}")
system(f"cd {new_day} && cargo add --path ../utils utils")

with open(Path("puzzles") / f"{new_day}.txt", "a"):
    pass

with open(Path("puzzles") / f"{new_day}_test.txt", "a"):
    pass

with open(Path(f"{new_day}") / f"puzzle.txt", "a"):
    pass

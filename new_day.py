#!/bin/env python3

from glob import glob
from os import system
from pathlib import Path

new_day = "day{}".format(int(sorted(glob("day*")).pop().replace("day",""))+1)
system(f"cargo init {new_day}")
system(f"cd {new_day} && cargo add --path ../utils utils")

with open(Path("puzzles")/f"{new_day}.txt","w"):
    pass

with open(Path("puzzles")/f"{new_day}_test.txt","w"):
    pass

with open(Path(f"{new_day}")/f"puzzle.txt","w"):
    pass

import toml

with open("Cargo.toml", "r") as conffile:
    config = toml.loads(conffile.read())
    members = config["workspace"]["members"]
    members.append(new_day)

with open("Cargo.toml", "w") as conffile:
    toml.dump(config, conffile)

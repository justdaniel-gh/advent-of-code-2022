#!/bin/env python3

from glob import glob
from os import system
from pathlib import Path

import toml


class TomlNoTrailingSeparatorEncoder(toml.TomlEncoder):
    """
    The default Encoder puts a ',' separator at the end of an array list, the VSCode Cargo parser doesn't like it
    """

    def dump_list(self, v):
        retval = "["
        retval += ", ".join(str(self.dump_value(u)) for u in v)
        retval += "]"
        return retval


new_day = "day{}".format(int(sorted(glob("day*")).pop().replace("day", "")) + 1)

with open("Cargo.toml", "r") as conf_file:
    config = toml.loads(conf_file.read())
    members = config["workspace"]["members"]
    members.append(new_day)

with open("Cargo.toml", "w") as conf_file:
    toml.dump(config, conf_file, encoder=TomlNoTrailingSeparatorEncoder())

system(f"cargo init {new_day}")
system(f"cd {new_day} && cargo add --path ../utils utils")

with open(Path("puzzles") / f"{new_day}.txt", "w"):
    pass

with open(Path("puzzles") / f"{new_day}_test.txt", "w"):
    pass

with open(Path(f"{new_day}") / f"puzzle.txt", "w"):
    pass

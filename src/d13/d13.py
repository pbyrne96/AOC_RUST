import re
import ast
from copy import deepcopy

parsed_data = []
STEP = 2

with open('src/d13/test_input.txt') as f:
    lines = f.readlines()
    hold = []
    for line in lines:
        _line = re.sub(r'\n', '', line)
        if not _line:
            continue

        parsed_line = ast.literal_eval(_line)
        hold.append(parsed_line)
        if len(hold) == STEP:
            parsed_data.append(list(deepcopy(hold)))
            hold.clear()


def compare_pairs(a,b):
    ...


def part_one(parsed_data):
    for line in parsed_data:
        left,right = tuple(line)
        print(left, right)


if __name__ == "__main__":
    part_one(parsed_data=parsed_data)
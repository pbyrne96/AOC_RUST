import re
import ast
from copy import deepcopy
from itertools import zip_longest

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
    print(a,b)
    for point_a, point_b in zip_longest(a,b):

        if isinstance(point_a, int) and isinstance(point_b, list):
            return compare_pairs([point_a], point_b)
        if isinstance(point_a, list) and isinstance(point_b, int):
            return compare_pairs(point_a, [point_b])

        if point_b is None and point_a:
            return False
        if point_a is None and point_b:
            return True
        if isinstance(point_a, int) and isinstance(point_b, int):
            if point_a == point_b:
                continue
            if point_a < point_b:
                return True

        if isinstance(point_a, list) and isinstance(point_b, list):
           continue
    return False



def part_one(parsed_data):
    right_order = 0

    for idx, line in enumerate(parsed_data):
        first,second = tuple(line)
        print(compare_pairs(first, second))


    return right_order


if __name__ == "__main__":
    print(part_one(parsed_data=parsed_data))
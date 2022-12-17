import json
import sys

test = False if len(sys.argv) < 2 or sys.argv[1] != 'test' else True

def check_if_in_right_order(left, right, left_index=0, right_index=0) -> bool:
    # print(f"{left = } {right = }")

    if left_index >= len(left) :
        return right_index < len(right)

    if left_index < len(left) and right_index >= len(right):
        return False

    right_order = True

    left_thing = left[left_index]
    right_thing = right[right_index]

    if isinstance(left_thing, list):
        # left is a list
        if isinstance(right_thing, list):
            if len(left) != 0 and len(right) != 0:
                right_order = check_if_in_right_order(left_thing, right_thing)
            else:
                right_order = check_if_in_right_order(left_thing, right_thing, left_index + 1, right_index + 1)
        else:
            right_order = check_if_in_right_order(left_thing, [right_thing])
    else:
        # left is an int
        if isinstance(right_thing, list):
            right_order =  check_if_in_right_order([ left_thing ], right_thing)
        else:
            if left_thing > right_thing:
                right_order = False
            elif left_thing < right_thing:
                right_order = True
            else:
                right_order =  check_if_in_right_order(left, right, left_index + 1, right_index + 1)



    return right_order

with open(f'src/inputs/{"test/" if test else ""}day13.txt') as file:
    i = 0
    lines = file.readlines()

    pair_number = 0
    total  = 0

    while i < len(lines) - 1:
        if len(lines[i].strip()) == 0:
            i += 1
            continue

        pair_number += 1

        left = json.loads(lines[i])
        right = json.loads(lines[i + 1])

        right_order = check_if_in_right_order(left, right)

        if pair_number == 130: print(f"{pair_number} are in right order {right_order}")

        if right_order:
            total += pair_number

        i += 2

    print(total)



# import functools
# import itertools
# import math
# 
# 
# def cmp_int(a, b):
#     return (a > b) - (a < b)
# 
# 
# def cmp(left, right):
#     match left, right:
#         case (int(left), int(right)):
#             return cmp_int(left, right)
# 
#         case (int(left), _):
#             left = [left]
# 
#         case (_, int(right)):
#             right = [right]
# 
#     for l, r in itertools.zip_longest(left, right):
#         if l is None:
#             return -1
#         elif r is None:
#             return 1
# 
#         c = cmp(l, r)
# 
#         if c != 0:
#             return c
#     return 0 
# 
# 
# def parse(expr):
#     stack = []
#     current = []
# 
#     parsing_number = False
# 
#     for char in expr:
#         if parsing_number and not char.isdigit():
#             number = int("".join(current))
#             current = stack.pop()
#             current.append(number)
#             parsing_number = False
# 
#         if char.isdigit():
#             if not parsing_number:
#                 parsing_number = True
#                 stack.append(current)
#                 current = []
#             current.append(char)
#         elif char == "[":
#             stack.append(current)
#             current = []
#         elif char == "]":
#             tmp = current
#             current = stack.pop()
#             current.append(tmp)
# 
# 
#     return current[0]
# 
# 
# def parse_lines(lines):
#     for line in lines:
#         if len(line) == 0:
#             continue
# 
#         # Too easy!
#         # yield ast.literal_eval(line)
# 
#         yield parse(line)
# 
# 
# lines = list(parse_lines(l.strip() for l in open("src/inputs/day13.txt").readlines()))
# 
# pairs = zip(lines[::2], lines[1::2])
# print(sum(index for index, packet in enumerate(pairs, start=1) if cmp(packet[0], packet[1]) < 0))
# 
# dividers = [[[2]], [[6]]]
# lines.extend(dividers)
# 
# sorted_packets = sorted(lines, key=functools.cmp_to_key(cmp))
# print(math.prod(i for i, e in enumerate(sorted_packets, start=1) if e in dividers))

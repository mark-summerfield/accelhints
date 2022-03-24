#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import collections
import math
import sys

import munkres


def main():
    filenames, show_quality = _get_args()
    _process(filenames, show_quality)


def _get_args():
    show_quality = False
    filenames = []
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit(USAGE)
        elif arg in {'-q', '--quality'}:
            show_quality = True
        else:
            filenames.append(arg)
    return filenames, show_quality


def _process(filenames, show_quality):
    if filenames:
        for name in filenames:
            with open(name, 'rt', encoding='utf-8') as file:
                for group in file.read().split('\n\n'):
                    lines = [line.strip() for line in group.splitlines()
                             if line.strip() and not line.startswith('#')]
                    _process_list(lines, show_quality)
    else:
        lines = []
        while True:
            try:
                line = input().strip()
                if line:
                    if not line.startswith('#'):
                        lines.append(line)
                else:
                    _process_list(lines, show_quality)
                    lines = []
            except EOFError:
                break
        _process_list(lines, show_quality)


def _process_list(lines, show_quality):
    if lines:
        for line in accelhints(lines):
            print(line)
        if show_quality:
            print(f'# Quality = {quality(lines):.0%}')
        print()


def accelhints(lines, *, alphabet='ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789'):
    weights = _get_weights(lines, alphabet)
    m = munkres.Munkres()
    weights = munkres.make_cost_matrix(weights)
    indexes = m.compute(weights)
    _update_lines(lines, alphabet, indexes)
    return lines


def _get_weights(lines, alphabet):
    size = len(alphabet)
    weights = [[0] * size for row in range(size)]
    for row, line in enumerate(lines):
        prev = None
        for column, c in enumerate(line):
            c = c.upper()
            i = alphabet.find(c)
            if i > -1: # c in alphabet
                if column == 0: # first
                    weight = 4
                elif prev == '&': # preset
                    weight = 99
                elif prev.isspace(): # word start
                    weight = 2
                else: # anywhere
                    weight = 1
                if weights[row][i] < weight:
                    weights[row][i] = weight
            prev = c
    return weights


def _update_lines(lines, alphabet, indexes):
    for row, column in indexes:
        c = alphabet[column]
        if row < len(lines):
            line = lines[row]
            i = line.find('&')
            if i > -1:
                continue # user preset
            uline = line.upper()
            index = None
            if uline[0] == c:
                index = 0 # first is best
            else:
                index = uline.find(f' {c}') # start of word is second best
                if index > -1:
                    index += 1 # skip the space
                else:
                    index = uline.find(c) # anywhere is third best
            if index is not None and index > -1:
                lines[row] = line[:index] + '&' + line[index:]


class Error(Exception):
    pass


def quality(lines):
    count_for_char = collections.defaultdict(int)
    target = 0.0
    actual = 0.0
    size = len(lines)
    for row, line in enumerate(lines):
        ideal = (25.0 * size) + row
        target += ideal
        if line.startswith('&'):
            actual += ideal
            count_for_char[line[1].upper()] += 1
        else:
            factor = None
            i = line.find(' &')
            if i > -1: # word start
                factor = 5.0
                i += 2 # move onto the &
            else:
                i = line.find('&')
                if i > -1: # anywhere
                    i += 1
                    if i == len(line):
                        raise Error('& at end of line')
                    factor = 1.0
            if factor is not None:
                actual += (factor * size) + row - i
                count_for_char[line[i].upper()] += 1
    if math.isclose(target, 0):
        raise Error('missing lines')
    for char, count in count_for_char.items():
        if count > 1:
            raise Error(f'&{char} occurs {count} times')
    return actual / target


USAGE = '''usage: accelhints.py [-q|--quality] items.txt >out.txt

The input is just plain text lines with one item per line and with any
preset acceleraters preceded by an ampersand.
If you want to have multiple lists (e.g., File menu, Edit menu, a dialog,
etc.), just separate each list with a blank line.
Comments may be included on lines that begin with '#'.

If the quality flag is present each list will be followed by a quality %.

Example input | Expected output
--------------+----------------
Undo          |&Undo
Redo          |&Redo
Copy          |&Copy
Cu&t          |Cu&t
Paste         |&Paste
Find          |&Find
Find Again    |Find &Again'''


if __name__ == '__main__':
    main()

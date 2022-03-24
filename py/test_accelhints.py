#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import os
import unittest

import accelhints

os.chdir(os.path.dirname(__file__))

PATH = '../tests'


class TestAccelHints(unittest.TestCase):

    def test(self):
        self.maxDiff = 0
        qualities = (
            0.75, 0.75, 0.71, 0.33, 0.58, 0.62, 0.72, 1, 1, 0.81, 0.65,
            0.48, 0.62, 0.64, 0.55, 0.61, 0.49, 0.55, 0.63, 0.55, 0.63,
            0.81, 0.53, 0.57, 0.67, 0.75, 0.62, 0.56, 0.57, 0.57, 0.49,
            0.0)
        inputs = list(read_lists(f'{PATH}/input.txt'))
        expecteds = list(read_lists(f'{PATH}/expected-py.txt'))
        self.assertEqual(len(inputs), len(expecteds))
        for i in range(len(inputs)):
            lines = inputs[i]
            if lines:
                with self.subTest(i=i):
                    actuals = accelhints.accelhints(lines)
                    self.assertEqual(
                        actuals, expecteds[i],
                        f'list #{i+1}:\nA={actuals}\nE={expecteds[i]}')
                    self.assertAlmostEqual(
                        qualities[i], accelhints.quality(actuals), places=2)


def read_lists(filename):
    with open(filename, 'rt', encoding='utf-8') as file:
        text = file.read()
    for list_of_items in text.split('\n\n'):
        yield [line for line in list_of_items.splitlines()
               if line.strip() and not line.startswith('#')]


if __name__ == '__main__':
    unittest.main()

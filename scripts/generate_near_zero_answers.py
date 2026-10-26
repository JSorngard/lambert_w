# /// script
# requires-python = ">=3.9"
# dependencies = ["mpmath==1.3.0"]
# ///
"""Regenerate the W0(F)_NEAR_ZERO_ANSWERS statics with `uv run scripts/generate_near_zero_answers.py`.

Inputs are binary64/binary32 values, converted exactly to mpmath. References
are evaluated independently at 100 and 180 decimal digits and must round to
the same target float. The CSV is consumed by ordinary Rust tests.
"""

import csv
import math
import struct
from pathlib import Path

import mpmath as mp


def f32(x: float):
    """Truncates a float value to 32-bits in the same way that C does and returns the resulting value"""
    return struct.unpack("f", struct.pack("f", x))[0]


tiny = float.fromhex("0x0.0000000000001p-1022")
normal = float.fromhex("0x1p-1022")
cutoff = float(0.05)
neighbors = [math.nextafter(cutoff, 0), cutoff, math.nextafter(cutoff, math.inf)]

values = [tiny, 2 * tiny, normal - tiny, normal, *neighbors]
values += [float(10.0**e) for e in [-300, -100, -30, -20, -12, -8, -6, -4, -3, -2]]
values += [float(i / 1000) for i in range(1, 50, 3)]
set = {sign * v for v in values if v != 0 for sign in [-1, 1]}
print(f"static W0_NEAR_ZERO_ANSWERS: [(f64, f64); {len(set)}] = [")
for x in sorted(set):
    with mp.workdps(100):
        expected = float(mp.lambertw(mp.mpf(x)))
    with mp.workdps(180):
        assert expected == float(mp.lambertw(mp.mpf(x)))
    print(f"    ({repr(x)}, {repr(expected)}),")
print("];")


tiny = 2.0**-149
normal = 2.0**-126
cutoff = f32(0.05)
bits = struct.unpack("I", struct.pack("f", cutoff))[0]
neighbors = [struct.unpack("f", struct.pack("I", bits + i))[0] for i in [-1, 0, 1]]

values = [tiny, 2 * tiny, normal - tiny, normal, *neighbors]
values += [f32(10.0**e) for e in [-300, -100, -30, -20, -12, -8, -6, -4, -3, -2]]
values += [f32(i / 1000) for i in range(1, 50, 3)]
set = {sign * v for v in values if v != 0 for sign in [-1, 1]}
print(f"static W0F_NEAR_ZERO_ANSWERS: [(f32, f32); {len(set)}] = [")
for x in sorted(set):
    with mp.workdps(100):
        expected = f32(mp.lambertw(mp.mpf(x)))
    with mp.workdps(180):
        assert expected == f32(mp.lambertw(mp.mpf(x)))
    print(f"    ({repr(x)}, {repr(expected)}),")
print("];")
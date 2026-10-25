# /// script
# requires-python = ">=3.9"
# dependencies = ["mpmath==1.3.0"]
# ///
"""Regenerate near_zero.csv with `uv run scripts/generate_near_zero.py`.

Inputs are binary64/binary32 values, converted exactly to mpmath. References
are evaluated independently at 100 and 180 decimal digits and must round to
the same target float. The CSV is consumed by ordinary Rust tests.
"""

import csv
import math
import struct
from pathlib import Path

import mpmath as mp


def f32(x):
    return struct.unpack("f", struct.pack("f", x))[0]


with Path(__file__).with_name("near_zero.csv").open("w", newline="") as output:
    writer = csv.writer(output, lineterminator="\n")
    writer.writerow(["type", "x", "w0"])
    for kind, cast, tiny, normal in [
        ("f64", float, float.fromhex("0x0.0000000000001p-1022"), float.fromhex("0x1p-1022")),
        ("f32", f32, 2.0**-149, 2.0**-126),
    ]:
        cutoff = cast(0.05)
        if kind == "f64":
            neighbors = [math.nextafter(cutoff, 0), cutoff, math.nextafter(cutoff, math.inf)]
        else:
            bits = struct.unpack("I", struct.pack("f", cutoff))[0]
            neighbors = [struct.unpack("f", struct.pack("I", bits + i))[0] for i in [-1, 0, 1]]
        values = [tiny, 2 * tiny, normal - tiny, normal, *neighbors]
        values += [cast(10.0**e) for e in [-300, -100, -30, -20, -12, -8, -6, -4, -3, -2]]
        values += [cast(i / 1000) for i in range(1, 50, 3)]
        for x in sorted({sign * v for v in values if v != 0 for sign in [-1, 1]}):
            with mp.workdps(100):
                expected = cast(mp.lambertw(mp.mpf(x)))
            with mp.workdps(180):
                assert expected == cast(mp.lambertw(mp.mpf(x)))
            writer.writerow([kind, repr(x), repr(expected)])

import re
import string
import random
import myrustlib  # <-- Importing Rust Implemented Library


def count_doubles(val):
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


double_re = re.compile(r'(?=(.)\1)')


def count_double_regex(val):
    return len(double_re.findall(val))


val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_regex(benchmark):
    benchmark(count_double_regex, val)


def test_rust(benchmark):
    benchmark(myrustlib.count_doubles, val)

def test_rust_once(benchmark):
    benchmark(myrustlib.count_doubles_once, val)

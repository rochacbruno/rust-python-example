# Speed up your Python using Rust

![Rust](https://www.rust-lang.org/logos/rust-logo-blk.svg)

## What is Rust?

**Rust** is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. 

Featuring

* zero-cost abstractions
* move semantics
* guaranteed memory safety
* threads without data races
* trait-based generics
* pattern matching
* type inference
* minimal runtime
* efficient C bindings

> Taken from: from rust-lang.org

## Why does it matter for a Python developer?

The better description of Rust I heard from [**Elias**](https://github.com/dlight) a member and the **Rust Guru** of the [**Rust Brazil Telegram Group**](https://t.me/rustlangbr)

> **Rust is** a language that allows you to build high level abstractions, but without giving up low level control - that is, control of how data is represented in memory, control of which threading model you want to use etc.  
> **Rust is** a language that can usually detect, during compilation, the worst parallelism and memory management errors (such as accessing data on different threads without synchronization, or using data after they have been deallocated), but gives you a hatch escape in the case you really know what you're doing.  
> **Rust is** a language that, because it has no runtime, can be used to integrate with any runtime; you can write a native extension in Rust that is called by a program node.js, or by a python program, or by a program in ruby, lua etc. and, on the other hand, you can script a program in Rust using these languages.  -- "Elias Gabriel Amaral da Silva"


![PyRust](https://user-images.githubusercontent.com/458654/32692578-9a424482-c701-11e7-8ea5-09c71612b96c.png)

There are a bunch of Rust packages out there to help you extending Python with Rust. 

I can mention [Milksnake](https://github.com/getsentry/milksnake) created by Armin Ronacher (the creator of Flask)  and also [PyO3](https://github.com/PyO3/pyo3) The Rust bindings for Python interpreter

> See a complete reference list at the bottom.

For this post, I am going to use [Rust Cpython](https://github.com/dgrunwald/rust-cpython), it's the only one I have tested, it is compatible with **stable version of Rust** and found it straightforward to use.

> **NOTE**: [PyO3](https://github.com/PyO3/pyo3) is a fork of rust-cpython, comes with many improvements, but works only with the nightly version of Rust, so I prefered to use the stable for this post, anyway the examples here must work also with PyO3.

**Pros:** It is really easy to write Rust functions and import from Python and as you will see by the benchmarks it worth in terms of performance.

**Cons:** The distribution of your **project/lib/framework** will demand the Rust module to be compiled on the target system because of variation of environment and architecture, there will be a **compiling** stage which you don't have when installing Pure Python libraries, you can make it easier using [rust-setuptools](https://pypi.python.org/pypi/setuptools-rust) or using the [MilkSnake](https://github.com/getsentry/milksnake) to embed binary data in Python Wheels.

## Python is sometimes slow

Yes, Python is known for being "slow" in some cases and the good news is that this doesn't really matter depending on your project goals and priorities. For most projects this 
detail will not be very important.

However, you may face the **rare** case where a single function or module is taking too much time and is detected as the bottleneck of your project performance, often happens with string parsing and image processing.

## Example

Lets say you have a Python function which does some kind of string processing, take the following easy example of `counting pairs of repeated chars` but have in mind that this example can be reproduced with other `string processing` functions or any other generally slow process in Python.


```bash
# How many subsequent-repeated group of chars are in the given string? 
abCCdeFFghiJJklmnopqRRstuVVxyZZ... {millions of chars here}
  1   2    3        4    5   6 
```

Python is pretty slow for doing large `string` processing so you can use `pytest-benchmark` to compare a `Pure Python (with Iterator Zipping)` function versus a `Regexp` implementation.

```bash
# Using a Python3.6 environment
$ pip3 install pytest pytest-benchmark

```

Then write a new Python program called `doubles.py`

```python
import re
import string
import random

# Python ZIP version
def count_doubles(val):
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


# Python REGEXP version
double_re = re.compile(r'(?=(.)\1)')

def count_doubles_regex(val):
    return len(double_re.findall(val))


# Benchmark it
# generate 1M of random letters to test it
val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))

def test_pure_python(benchmark):
    benchmark(count_doubles, val)

def test_regex(benchmark):
    benchmark(count_doubles_regex, val)
```

Run **pytest** to compare:


```bash
$ pytest doubles.py                                                                                                           
================================================================================= test session starts ==================================================================================
platform linux -- Python 3.6.0, pytest-3.2.3, py-1.4.34, pluggy-0.4.0
benchmark: 3.1.1 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Projects/rustpy, inifile:
plugins: benchmark-3.1.1
collected 2 items

doubles.py ..


--------------------------------------------------------------------------------- benchmark: 2 tests --------------------------------------------------------------------------------
Name (time in ms)         Min                Max               Mean            StdDev             Median               IQR            Outliers      OPS            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_regex            24.6824 (1.0)      32.3960 (1.0)      27.0167 (1.0)      1.8610 (1.0)      27.2148 (1.0)      2.9345 (4.55)         16;1  37.0141 (1.0)          36           1
test_pure_python      51.4964 (2.09)     62.5680 (1.93)     52.8334 (1.96)     2.3630 (1.27)     52.2846 (1.92)     0.6444 (1.0)           1;2  18.9274 (0.51)         20           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
=============================================================================== 2 passed in 4.10 seconds ===============================================================================

```

Lets take the `Median` for comparison:

- **Regexp** - 27.2148    **<-- less is better**
- **Python Zip** - 52.2846  

# Extending Python with Rust

# Create a new crate

> **crate** is how we call Rust Packages.

Having rust installed (recommended way is https://www.rustup.rs/)

> I used `rustc 1.21.0`


In the same folder run:

```bash
cargo new pyext-myrustlib
```

It creates a new Rust project in that same folder called `pyext-myrustlib` containing the `Cargo.toml` (cargo is the Rust package manager) and also a `src/lib.rs` (where we write our library implementation)

# Edit Cargo.toml

It will use the `rust-cpython` crate as dependency and tell cargo to generate a `dylib` to be imported from Python

```toml
[package]
name = "pyext-myrustlib"
version = "0.1.0"
authors = ["Bruno Rocha <rochacbruno@gmail.com>"]

[lib]
name = "myrustlib"
crate-type = ["dylib"]

[dependencies.cpython]
version = "0.1"
features = ["extension-module"]
```  

# Edit src/lib.rs

What we need to do:

1) Import all macros from `cpython` crate
2) Take `Python` and `PyResult` types from cpython in to our lib scope
3) Write the `count_doubles` function implementation in `Rust`, note that this is very similar to the Pure Python version except for:

    * It takes a `Python` as first argument, which is a reference to the Python Interpreter and allows Rust to use the `Python GIL`
    * Receives a `&str` typed `val` as reference
    * Returns a `PyResult` which is a type that allows the raise of Python exceptions
    * Returns a `PyResult` object in `Ok(total)` (**Result** is a enum type that represents either success (Ok) or failure (Err)) and as our function is expected to return a `PyResult` the compiler will take care of **wrapping** our `Ok` on that type. (note that our PyResult expects a `u64` as return value)

4) Using `py_module_initializer!` macro we register new attributes to the lib, including the `__doc__` and also we add the `count_doubles` attribute referencing our `Rust implementation of the function`
    * Attention to the names **lib**myrustlib, **initlib**myrustlib and **PyInit**_myrustlib which is suffixed by our library name (defined in Cargo.toml)
    * We also use the `try!` macro which is the equivalent to Python's `try.. except`
    * Return `Ok(())` - The `()` is an empty result tuple, the equivalent of `None` in Python

```rust
#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(total)
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m | {
    try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    try!(m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str))));
    Ok(())
});

```

Now lets build it in cargo

```bash
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs

$ ls -la target/release/libmyrustlib*
target/release/libmyrustlib.d
target/release/libmyrustlib.so*  <-- Our dylib is here
```

Now lets copy the generated `.so` lib to the same folder where our `doubles.py` is:

> NOTE: on **Fedora** you must get a `.so` in other system you may get a `.dylib` and you can rename it changing extension to `.so`

```bash
$ cd ..
$ ls
doubles.py pyext-myrustlib/

$ cp pyext-myrustlib/target/release/libmyrustlib.so myrustlib.so

$ ls
doubles.py myrustlib.so pyext-myrustlib/
```

> Having the `myrustlib.so` in the same folder or added to your Python path allows it to be directly imported, transparently as it was a Python module.


# Importing from Python and comparing the results

Edit your `doubles.py` now importing our `Rust implemented` version and also adding a `benchmark` for it.


```python
import re
import string
import random
import myrustlib   #  <-- Import the Rust implemented module (myrustlib.so)


def count_doubles(val):
    """Count repeated pair of chars ins a string"""
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


double_re = re.compile(r'(?=(.)\1)')


def count_doubles_regex(val):
    return len(double_re.findall(val))


val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_regex(benchmark):
    benchmark(count_doubles_regex, val)


def test_rust(benchmark):   #  <-- Benchmark the Rust version
    benchmark(myrustlib.count_doubles, val)

```

# Benchmark

```bash
$ pytest doubles.py
================================================================================= test session starts ==================================================================================
platform linux -- Python 3.6.0, pytest-3.2.3, py-1.4.34, pluggy-0.4.0
benchmark: 3.1.1 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Projects/rustpy, inifile:
plugins: benchmark-3.1.1
collected 3 items

doubles_rust.py ...


--------------------------------------------------------------------------------- benchmark: 3 tests ---------------------------------------------------------------------------------
Name (time in ms)         Min                Max               Mean            StdDev             Median               IQR            Outliers       OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_rust              2.5555 (1.0)       2.9296 (1.0)       2.6085 (1.0)      0.0521 (1.0)       2.5935 (1.0)      0.0456 (1.0)         53;23  383.3661 (1.0)         382           1
test_regex            25.6049 (10.02)    27.2190 (9.29)     25.8876 (9.92)     0.3543 (6.80)     25.7664 (9.93)     0.3020 (6.63)          4;3   38.6285 (0.10)         40           1
test_pure_python      52.9428 (20.72)    56.3666 (19.24)    53.9732 (20.69)    0.9248 (17.75)    53.6220 (20.68)    1.4899 (32.70)         6;0   18.5277 (0.05)         20           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
=============================================================================== 3 passed in 5.19 seconds ===============================================================================

```

Lets take the `Median` for comparison:

- **Rust** - 2.5935    **<-- less is better**
- **Regexp** - 25.7664
- **Python Zip** - 53.6220

Rust implementation can be **10x** faster than Python Regex and **21x** faster than Pure Python Version.

> Interesting that **Regex** version is only 2x faster than Pure Python :)

> NOTE: That numbers makes sense only for this particular scenario, for other cases that comparison may be different.

# Conclusion

`Rust` may not be **yet** the `general purpose language` of choice by its level of complexity and may not be the better choice **yet** to write common simple `applications` such as `web` sites and `test automation` scripts.

However, for `specific parts` of the project where Python is known to be the bottleneck and your natural choice would be implementing a `C/C++` extension, writing this extension in Rust seems easy and better to maintain.

There are still many improvements to come in Rust and lots of others crates to offer `Python <--> Rust` integration. Even if your are not including the language in your tool belt right now, it is really worth to keep an eye open to the future!

## Credits

The examples on this publication are inspired by `Extending Python with Rust` talk by **Samuel Cormier-Iijima** in **Pycon Canada**.
video here: https://www.youtube.com/watch?v=-ylbuEzkG4M

And also by `My Python is a little Rust-y` by **Dan Callahan** in **Pycon Montreal**.
video here: https://www.youtube.com/watch?v=3CwJ0MH-4MA

Other references:

- https://github.com/mitsuhiko/snaek
- https://github.com/PyO3/pyo3
- https://pypi.python.org/pypi/setuptools-rust
- https://github.com/mckaymatt/cookiecutter-pypackage-rust-cross-platform-publish
- http://jakegoulding.com/rust-ffi-omnibus/
- https://github.com/urschrei/polylabel-rs/blob/master/src/ffi.rs
- https://bheisler.github.io/post/calling-rust-in-python/
- https://github.com/saethlin/rust-lather

Join Community:

Join Rust community, you can find group links in https://www.rust-lang.org/en-US/community.html

**If you speak Portuguese** I recommend you to join https://t.me/rustlangbr and there
is also the http://bit.ly/canalrustbr on Youtube.

## Author

**Bruno Rocha**
- Senior Quality Enginner at **Red Hat**
- Teaching Python at CursoDePython.com.br
- Fellow Member of Python Software Foundation

More info: http://about.me/rochacbruno and http://brunorocha.org

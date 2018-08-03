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
=============================================================================
platform linux -- Python 3.6.0, pytest-3.2.3, py-1.4.34, pluggy-0.4.
benchmark: 3.1.1 (defaults: timer=time.perf_counter disable_gc=False min_roun
rootdir: /Projects/rustpy, inifile:
plugins: benchmark-3.1.1
collected 2 items

doubles.py ..


-----------------------------------------------------------------------------
Name (time in ms)         Min                Max               Mean          
-----------------------------------------------------------------------------
test_regex            24.6824 (1.0)      32.3960 (1.0)      27.0167 (1.0)    
test_pure_python      51.4964 (2.09)     62.5680 (1.93)     52.8334 (1.96)   
-----------------------------------------------------------------------------

```

Lets take the `Mean` for comparison:

- **Regexp** - 27.0167    **<-- less is better**
- **Python Zip** - 52.8334  

# Extending Python with Rust

# Create a new crate

> **crate** is how we call Rust Packages.

Having rust installed (recommended way is https://www.rustup.rs/) 
Rust is also available on Fedora and RHEL [rust-toolset](https://developers.redhat.com/blog/2017/11/01/getting-started-rust-toolset-rhel/)

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
==============================================================================
platform linux -- Python 3.6.0, pytest-3.2.3, py-1.4.34, pluggy-0.4.
benchmark: 3.1.1 (defaults: timer=time.perf_counter disable_gc=False min_round
rootdir: /Projects/rustpy, inifile:
plugins: benchmark-3.1.1
collected 3 items

doubles_rust.py ...


-----------------------------------------------------------------------------
Name (time in ms)         Min                Max               Mean          
-----------------------------------------------------------------------------
test_rust              2.5555 (1.0)       2.9296 (1.0)       2.6085 (1.0)    
test_regex            25.6049 (10.02)    27.2190 (9.29)     25.8876 (9.92)   
test_pure_python      52.9428 (20.72)    56.3666 (19.24)    53.9732 (20.69)  
-----------------------------------------------------------------------------
```

Lets take the `Mean` for comparison:

- **Rust** - 2.6085    **<-- less is better**
- **Regexp** - 25.8876
- **Python Zip** - 53.9732

Rust implementation can be **10x** faster than Python Regex and **21x** faster than Pure Python Version.

> Interesting that **Regex** version is only 2x faster than Pure Python :)

> NOTE: That numbers makes sense only for this particular scenario, for other cases that comparison may be different.


# Updates and improvements

After this article has been published I got some comments on [r/python](https://www.reddit.com/r/Python/comments/7dct9v/use_rust_to_write_python_modules/)
and also on [r/rust](https://www.reddit.com/r/rust/comments/7dctmp/red_hat_developers_blog_speed_up_your_python/) 

The contributions come as [Pull Requests](https://github.com/rochacbruno/rust-python-example/pulls?utf8=%E2%9C%93&q=is%3Apr) and you can send a new if you think the functions can be improved.

Thanks to: [Josh Stone](https://github.com/cuviper) we got a better implementarion for Rust which iterates the string only once and also the Python equivalent.

Thanks to: [Purple Pixie](https://github.com/purple-pixie) we got a Python implementation using `itertools`, however this version is not performing any better, needs improvements.


## Iterating only once

```rust
fn count_doubles_once(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.chars();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }

    Ok(total)
}
```

```python
def count_doubles_once(val):
    total = 0
    chars = iter(val)
    c1 = next(chars)
    for c2 in chars:
        if c1 == c2:
            total += 1
        c1 = c2
    return total
```


## Python with Itertools

```python
import itertools

def count_doubles_itertools(val):
    c1s, c2s = itertools.tee(val)
    next(c2s, None)
    total = 0
    for c1, c2 in zip(c1s, c2s):
        if c1 == c2:
            total += 1
    return total
```

### Why not C/C++/Nim/Go/Ĺua/PyPy/{other language}?

Ok, that is not the purpose of this post, this post was never about comparing `Rust` X `other language`, this post was specifically about
**how to use Rust to extend and speed up Python** and by doing that it means you have a good reason to choose Rust instead of `other language`
or by its ecosystem or by its safety and tooling or just to follow the hype, or simply because you like Rust
doesn't matter the reason, this post is here to show how to use it with **Python**.

I (personally) may say that Rust is more `future proof` as it is new and there are lots of improvements to come, also because of its ecosystem, tooling and community
and also because I feel comfortable with Rust syntax, I really like it!

So, as expected people started complaining about the use of other languages and it becomes a sort of benchmark, and I think it is cool! 

So as part of my request for improvements some people on [Hacker News](https://news.ycombinator.com/item?id=15719254) also sent ideas, [martinxyz](https://github.com/martinxyz) sent an implementaion using
C and SWIG taht performed very well.

C Code (swig boilerplate ommited)

```c
uint64_t count_byte_doubles(char * str) {
  uint64_t count = 0;
  while (str[0] && str[1]) {
    if (str[0] == str[1]) count++;
    str++;
  }
  return count;
}
```

And our fellow Red Hatter [Josh Stone](https://github.com/cuviper) improved the Rust implementation again by replacing `chars` with `bytes` so it is a fair competition with `C` as
C is comparing bytes instead of unicode chars.


```rust
fn count_doubles_once_bytes(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    let mut chars = val.bytes();
    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }

    Ok(total)
}
```

There are also ideas to compare Python `list comprehension` and `numpy` so I included here

Numpy:

```python
import numpy as np

def count_double_numpy(val):
    ng=np.fromstring(val,dtype=np.byte)
    return np.sum(ng[:-1]==ng[1:])
```

List comprehension

```python
def count_doubles_comprehension(val):
    return sum(1 for c1, c2 in zip(val, val[1:]) if c1 == c2)
```

The complete test case is on repository `test_all.py` file.


## New Results

```bash
-------------------------------------------------------------------------------------------------
Name (time in us)                     Min                    Max                   Mean          
-------------------------------------------------------------------------------------------------
test_rust_bytes_once             476.7920 (1.0)         830.5610 (1.0)         486.6116 (1.0)    
test_c_swig_bytes_once           795.3460 (1.67)      1,504.3380 (1.81)        827.3898 (1.70)   
test_rust_once                   985.9520 (2.07)      1,483.8120 (1.79)      1,017.4251 (2.09)   
test_numpy                     1,001.3880 (2.10)      2,461.1200 (2.96)      1,274.8132 (2.62)   
test_rust                      2,555.0810 (5.36)      3,066.0430 (3.69)      2,609.7403 (5.36)   
test_regex                    24,787.0670 (51.99)    26,513.1520 (31.92)    25,333.8143 (52.06)  
test_pure_python_once         36,447.0790 (76.44)    48,596.5340 (58.51)    38,074.5863 (78.24)  
test_python_comprehension     49,166.0560 (103.12)   50,832.1220 (61.20)    49,699.2122 (102.13) 
test_pure_python              49,586.3750 (104.00)   50,697.3780 (61.04)    50,148.6596 (103.06) 
test_itertools                56,762.8920 (119.05)   69,660.0200 (83.87)    58,402.9442 (120.02) 
-------------------------------------------------------------------------------------------------

```

- The `new Rust implementation comparing bytes` is **2x better** than the old comparing unicode `chars`
- The `Rust` version is still better than the `C` using SWIG
- `Rust` comparing `unicode chars` is still better than `numpy`
- However `Numpy` is better than the `first Rust implementation` which had the problem of **double iteration over the unicode chars**
- Using a `list comprehension` does not make significative difference than using `pure Python`

> NOTE: If you want to propose changes or improvements send a PR here: https://github.com/rochacbruno/rust-python-example/


I received more contributions as Pull Requests one of then was by [Jason Knight](https://github.com/binarybana) to improve `Rust` using 

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

And for those who were curious about a comparison with `numba` so [Shyba](https://github.com/shyba) implemented it and it is available in the numba branch https://github.com/rochacbruno/rust-python-example/tree/numba.

```python
from numba import jit

@jit(nopython=True, cache=True)
def count_doubles_once_numba(val):
    total = 0
    chars = iter(val)
    c1 = next(chars)
    for c2 in chars:
        if c1 == c2:
            total += 1
        c1 = c2
    return total
```

Look the new results with **numba** at the top, pretty close to **Rust**

```bash
----------------------------------------------------------------------------------------------------
Name (time in us)                       Min                    Max                   Mean           
----------------------------------------------------------------------------------------------------
test_pure_python_once_numba        292.0990 (1.0)         317.7590 (1.0)         296.7477 (1.0)     
test_numpy_numba                   326.2470 (1.12)        526.1350 (1.66)        338.1704 (1.14)    
test_rust_bytes_once               336.0620 (1.15)      1,053.0090 (3.31)        342.5122 (1.15)    
test_c_swig_bytes_once             375.6310 (1.29)      1,389.9070 (4.37)        388.9181 (1.31)    
test_rust_once                     986.0360 (3.38)      2,498.5850 (7.86)      1,006.5819 (3.39)    
test_numpy                       1,137.1750 (3.89)      2,000.5430 (6.30)      1,167.2551 (3.93)    
test_rust                        2,555.1400 (8.75)      3,645.3900 (11.47)     2,592.0419 (8.73)    
test_regex                      22,597.1750 (77.36)    25,027.2820 (78.76)    22,851.8456 (77.01)   
test_pure_python_once           32,418.8830 (110.99)   34,818.0800 (109.57)   32,756.3244 (110.38)  
test_pure_python                43,823.5140 (150.03)   45,961.8460 (144.64)   44,367.1028 (149.51)  
test_python_comprehension       46,360.1640 (158.71)   50,578.1740 (159.17)   46,986.8058 (158.34)  
test_itertools                  49,080.8640 (168.03)   51,016.5230 (160.55)   49,405.2562 (166.49)  
----------------------------------------------------------------------------------------------------
```

And there is also a **Cython** implementation by [MIke Fletcher](https://github.com/mcfletch) in the branch `cython` https://github.com/rochacbruno/rust-python-example/tree/cython 

with the results:


```bash
----------------------------------------------------------------------------------------------------
Name (time in us)                       Min                    Max                   Mean           
----------------------------------------------------------------------------------------------------
test_rust_bytes_once             336.7590 (1.0)         806.2610 (1.0)         346.5317 (1.0)       
test_cython                      756.1610 (2.25)      2,343.3680 (2.91)        785.6455 (2.27)      
test_c_swig_bytes_once           802.4250 (2.38)      1,632.4290 (2.02)        840.8603 (2.43)      
----------------------------------------------------------------------------------------------------
```


# Conclusion

Back to the purpose of this post **How to Speed Up your Python with Rust** we started with:

- **Pure Python** function taking **102 **.
- Improved with **Numpy** (which is implemented in C) to take **3**.
- Ended with **Rust** taking **1** (just like numba version).

In this example **Rust** performed **100x** faster than our **Pure Python**.

`Rust` will not magically save you, you must know the language to be able to implement the clever solution and once implemented in the right it worth as much as C in terms of performance and also comes with amazing tooling, ecosystem, community and safety bonuses. 

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

.PHONY: clean clean-test clean-pyc clean-build docs help test-python test-rust test-c test-cython compile-cython compile-rust compile-c

clean: clean-build clean-pyc clean-test ## remove all build, test, coverage and Python artifacts

clean-build: ## remove build artifacts
	rm -fr build/
	rm -fr dist/
	rm -fr .eggs/
	rm -fr .compiled
	find . -name '*.egg-info' -exec rm -fr {} +
	find . -name '*.egg' -exec rm -f {} +

clean-pyc: ## remove Python file artifacts
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +
	find . -name '__pycache__' -exec rm -fr {} +

clean-test: ## remove test and coverage artifacts
	rm -fr .tox/
	rm -f .coverage
	rm -fr htmlcov/

lint: ## check style with flake8
	flake8 manage tests

test-python: ## run tests quickly with the default Python
	py.test -v -s doubles.py

test-rust: ## run tests quickly with the default Python
	py.test -v -s doubles_with_rust.py

test-c: ## run tests quickly with the default Python
	py.test -v -s doubles_with_c_swig.py

test-cython: # run tests quickly with the default Python
	py.test -v -s doubles_with_cython.py

test-all: ## run tests quickly with the default Python
	py.test -v -s doubles_all.py

compile-rust: ## compile new rust lib
	@cd pyext-myrustlib;RUSTFLAGS="-C target-cpu=native" cargo build --release
	@cp pyext-myrustlib/target/release/libmyrustlib.so myrustlib.so

compile-c: ## compile new c lib
	@cd pyext-myclib;python3 setup.py build_ext -i

compile-cython: ## compile new cython lib
	@cd pyext-mycythonlib;cythonize -a -i mycythonlib.pyx
	@cp pyext-mycythonlib/mycythonlib*.so ./

compile-all: compile-rust compile-c compile-cython

.compiled: compile-all
	touch .compiled

test: compile-all test-all

from distutils.core import setup, Extension

setup(ext_modules=[
    Extension('_myclib',
        sources=['myclib.i', 'myclib.c'],
        depends=['setup.py', 'mylib.h']
        )
])

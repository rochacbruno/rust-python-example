from distutils.core import setup, Extension


setup(
    ext_modules=[
        Extension(
            '_myclib',
            sources=['myclib.i', 'myclib.c'],
            extra_compile_args=["-march=native","-O3"],
            depends=['setup.py', 'mylib.h']
        )
    ]
)

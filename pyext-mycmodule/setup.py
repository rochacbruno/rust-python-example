from setuptools import Extension, setup

ext = Extension(
    name='mycmodule',
    sources=['./mycmodule.cpp'],
    extra_compile_args=['-fpermissive'],
)

setup(
    name='mycmodule',
    version='0.1.0',
    ext_modules=[ext],
)

#!/usr/bin/env python

import os

from setuptools import setup
from setuptools_rust import Binding, RustExtension

LONG_DESCRIPTION = """
Tool to build exploit. Works also in Python.
"""

rust_ext = RustExtension(
    target="cryptatools-python3.cryptatoolsffi",
    path="./Cargo.toml",
    binding=Binding.NoBinding,
)

setup(
    name='cryptatools-python3',
    version='0.1.0.dev0',
    description="The Python language bindings for Cryptatools.",
    long_description=LONG_DESCRIPTION,
    long_description_content_type='text/markdown',
    rust_extensions=[rust_ext],
    zip_safe=False,
    packages=['cryptatools-python3'],
    package_dir={'cryptatools-python3': './src/api/python3'},
    url="https://github.com/bitcoindevkit/bdk-python",
    author="Alekos Filini <alekos.filini@gmail.com>, Steve Myers <steve@notmandatory.org>",
    license="MIT or Apache 2.0",
)
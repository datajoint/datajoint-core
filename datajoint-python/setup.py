#!/usr/bin/env python
from setuptools import setup, find_packages
from os import path
import sys

min_py_version = (3, 6)

if sys.version_info < min_py_version:
    sys.exit('DataJoint is only supported for Python {}.{} or higher'.format(
        *min_py_version))

here = path.abspath(path.dirname(__file__))

sys.path.insert(0, path.abspath(path.join(path.dirname(
    __file__), 'datajoint'))) or sys.path.insert(0, (path.join(here, 'datajoint')))

long_description = "A relational data framework for scientific data pipelines with MySQL backend."

# read in version number into __version__
with open(path.join(here, 'datajoint', 'version.py')) as f:
    exec(f.read())

with open(path.join(here, 'requirements.txt'), encoding='utf8') as f:
    requirements = f.read().split()


setup(
    name="datajoint",
    version=__version__,
    description="A relational data pipeline framework.",
    long_description=long_description,
    url="https://github.com/datajoint/datajoint-core",
    packages=find_packages(exclude=['contrib', 'docs', 'tests*']),
    install_requires=requirements,
    python_requires='~={}.{}'.format(*min_py_version),
    setup_requires=["cffi>=1.0.0"],
    cffi_modules=[
        "./datajoint/datajoint-core/build_datajoint_core.py:ffi",
    ],
)

# -*- coding:utf-8 -*-
#!/usr/bin/env Python

import os, setuptools

if os.path.exists("README.md"):
    with open("README.md", "r", encoding = 'utf-8') as fh:
        long_description = fh.read()
else:
    long_description = ""

def load_requirements(base = None):
    file_path = os.path.abspath(os.path.join(
        os.path.dirname(__file__), 
        "req",
        f'{base}-requirements.txt' if base else 'requirements.txt'
    ))
    rvs = []
    with open(file_path, mode='r', encoding='utf-8') as f:
        rvs = [l.strip() for l in f.readlines()]
    return rvs

setuptools.setup(
    name="bm_fileio",
    version='0.0.1',
    author='xmyeen',
    author_email="xmyeen@126.com",
    description='',
    long_description=long_description,
    long_description_content_type="text/markdown",
    packages=setuptools.find_packages(exclude=["*.tests", "*.tests.*", "tests.*", "tests"]),
    platforms=["all"],
    classifiers = [
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Operating System :: OS Independent"
    ],
    python_requires='>=3.8',
    install_requires = load_requirements()
)

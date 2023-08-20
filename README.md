# mcq_solver
###### A Python binding for a Rust library to solve MCQ's from answer sheets and their grades

## Prerequisites

For now the only supported Python implementation is the popular CPython. Support for PyPy could eventually be added. In theory, __CPython version 3.10 or more__ is required but it could be less restrictive in practice.

The library was tested on CPython 3.10.6 as of now.

## Installation from pre-compiled wheels

These will eventually be added later to [PyPI](https://pypi.org/). For now, they can be found in the `github-pages` branch of the repository. Most architectures and OS are supported (Linux s390x is not). Once the wheel is downloaded, use `pip install path/to/wheel` or `conda install path/to/wheel` to add it to your Python/Anaconda environment.

## Uninstallation

`pip uninstall mcq-solver` or `conda uninstall mcq-solver`

## Examples

Some examples can be found in the `examples` directory. Each contain a `create_mcq.py` file that generates random MCQ's and an `example.py` file that uses a function of the library to solve it. `examples/basic` uses `solve` and `examples/reading_sheets` uses `solve_from_files`.

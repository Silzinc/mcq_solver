# mcq_solver
###### A Python binding for a Rust library to solve MCQ's from answer sheets and their grades

## Installation

### Option I. Pre-compiled wheels
These will soon be added to [PyPI](https://pypi.org/) hopefully. For now, they can be found in the `dist` directory of the repository. Only the __Linux__ (tested) and __Windows__ (not tested) versions are available at the moment. Once the wheel is downloaded, use `pip install path/to/wheel` to add it to your Python environment.

### Option II. Compile from source code
<p style="text-decoration: underline">This is the mandatory way for Mac OS users </p>

##### Prerequisites:
* A [Python](https://www.python.org/downloads/) interpreter with version ≥ 3.10 (tested on 3.10.6 as of now) with [`pip`](https://pip.pypa.io/en/stable/installation/) or [`conda`](https://www.anaconda.com/download) installed
* A [Rust](https://www.rust-lang.org/tools/install) compiler with the `cargo` package manager (usually installed simultaneously with the terminal)
* The [`git`](https://github.com/git-guides/install-git) terminal command

##### Steps to compilation:

1. Add the python package [`maturin`](https://pypi.org/project/maturin/0.8.2/) to your Python environment:
   ```
   pip install maturin
   ```
   Or
   ```
   conda install maturin
   ```
2. Download the source code:
   ```
   git clone https://github.com/Silzinc/mcq_solver.git
   ```
   This will create a folder `mcq_solver` and download the source code into it. 
   
3. Enter the `mcq_solver` folder:
   ```
   cd mcq_solver
   ```
4. Compile the source code
   ```
   maturin build --release
   ```
   This can take a few seconds.

After the compilation has ended, the terminal should mention a path to a ` .whl` file (the wheel). It is normally in ` mcq_solver/target/wheel`. Now, to install it, run `pip install path/to/wheel` or `conda install path/to/wheel`. To make sure the process was successful, check if `abi3` appears in the name of the wheel, as well as the keyword associated to your platform (`darwin` for Mac OS, `manylinux` for Linux and `win` for Windows).

## Uninstallation

`pip uninstall mcq-solver` or `conda uninstall mcq-solver`

## Examples

Some examples can be found in the `examples` directory. Each contain a `create_mcq.py` file that generates random MCQ's and an `example.py` file that uses a function of the library to solve it. `examples/basic` uses `solve` and `examples/reading_sheets` uses `solve_from_files`.

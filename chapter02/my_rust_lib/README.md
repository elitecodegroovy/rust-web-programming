
maturin:

Maturin has a develop subcommand which will compile our Rust code and install the
resulting Python module for immediate use. It has one caveat: we must run it from within a
Python virtual environment. We will not linger on virtual environments, but know that they
are used for dependency isolation in Python projects. This is to prevent users from
accidentally overwriting a globally installed (possibly stable) version of their package while
it’s still being developed.

dev steps:
- pip install maturin
- maturin develop (`maturin build --release` for a release build)
- python cmd like 'python main.py'

# Rust-Euler

![example workflow](https://github.com/stevenprimeaux/rust-euler/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/github/stevenprimeaux/rust-euler/graph/badge.svg?token=FEDqRwkEOR)](https://codecov.io/github/stevenprimeaux/rust-euler)

Rust-Euler is a library that provides efficient algorithms for solving math puzzles -- written in Rust and continually refactored into a set of general-purpose tools to make future puzzles easier. The goal is to solve as many Project Euler challenges as possible while making use of the best features of idiomatic Rust and avoiding code duplication.

Functionality includes:
* Commonly used mathematical functions, including overflow tools for large numbers and operations related to factorization and prime numbers.
* String manipulation tools, including functions to convert back and forth between grids of text strings and numeric vectors.
* Functionality needed to solve specific Project Euler challenges, like generating Collatz sequences or working with leap years.

Code coverage is provided by Tarpaulin.

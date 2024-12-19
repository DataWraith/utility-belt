# Utility-Belt

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

This is my Rust library of potentially useful tools for Advent of Code.

## Using

Add `utility-belt` to your `Cargo.toml`:

```toml
[dependencies]
utility-belt = { git = "https://github.com/DataWraith/utility-belt.git" }
```

In your Advent of Code projects, simply `use utility_belt::prelude::*;`.

## What's currently here?

### Re-exports

Most of the heavy-lifting is done by the other libraries this crate re-exports:

- [ahash](https://docs.rs/ahash) - fast HashMap and HashSet implementation
- [glam](https://docs.rs/glam) - convenient 2D vector types
- [indoc](https://docs.rs/indoc) - exposes macros for convenient inline String formatting
- [itertools](https://docs.rs/itertools) - various tools for working with iterators
- [ndarray](https://docs.rs/ndarray) - n-dimensional container for general elements
- [rangetools](https://docs.rs/rangetools) - Useful extension to ranges
- [rstest](https://docs.rs/rstest) - table-driven testing (very useful for AoC!) and fixtures
- [tinyvec](https://docs.rs/tinyvec) - stack-allocated (small) vectors

### Advent of Code-specific tools

- **Grid2D**, a convenient 2D grid backed by `ndarray::Array2`. It comes with
  the ability to parse grids from the usual puzzle-input format for grids and
  provides various utility functions for working with 2D grids.

  Comes with various ancillary structs (e.g. `Direction` and `Coordinate`) to
  make working with grids easier.

- **BorderedGrid2D**, a wrapper around Grid2D that adds an implicit border
  around the grid. This is sometimes useful, for example if you need to find
  all tiles connected to the outside of the grid.

- A few useful **math functions** (`gcd` and `lcm` for now), a few helpers for
  working with polynomials.

- Functions for calculating **area** of a simple polygon and determining whether
  or not a **point is inside of a polygon**.

- **Cumulative sum** helpers in 1D (`PrefixSum`, `FenwickTree`) and 2D (`SummedAreaTable`).

  These allow you to quickly look up the sum of values in a given 1D range or 2D
  rectangle.

- **bisection search** function

- **union-find datastructure** for easy connected components analysis

- **path contraction** for iterating a function millions of times, provided that
  there are cycles in the state-space path the function induces.

- **state iteration**

  The idea is to have a HashMap containing the current states. Then a transition
  function is applied to each state, and the resulting state(s) are collected into
  a new HashMap.

  The HashMap keeps track of how often a given state has occurred. This can be
  used to, for example, count how often a state is visited in a finite state
  machine after `n` iterations.

- an implementation of **branch and bound**

- Small **bitsets** for 8, 16, 32, 64 and 128 values

- **Optimization algorithms** (Beam Search)

- Solving equation systems using **Gauss-Jordan elimination**


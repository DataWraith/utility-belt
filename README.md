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

- [ahash](https://docs.rs/ahash) - fast HashMap and HashSet implementation, hashing of single values
- [glam](https://docs.rs/glam) - convenient 2D vector types
- [indoc](https://docs.rs/indoc) - exposes macros for convenient inline String formatting
- [itertools](https://docs.rs/itertools) - various tools for working with iterators
- [ndarray](https://docs.rs/ndarray) - n-dimensional container for general elements
- [nom](https://docs.rs/nom) and [nom_locate](https://docs.rs/nom_locate/latest/nom_locate/) - ergonomic parser combinators
- [pathfinding](https://docs.rs/pathfinding) - BFS, A*, Brent's algorithm for cycle finding, connected components, etc.
- [ranges](https://docs.rs/ranges) - range union/intersection, etc.
- [rayon](https://docs.rs/rayon) - parallel iterators
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

- A few useful **math functions** (`gcd` and `lcm` for now)

- Functions for calculating **area** of a simple polygon and determining whether
  or not a **point is inside of a polygon**.

- **Cumulative sum** helpers in 1D (`PrefixSum`) and 2D (`SummedAreaTable`).

  These allow you to quickly look up the sum of values in a given 1D range or 2D
  rectangle.

- **`nom` parsers**

  A few additional nom parsers that may be useful.

- **search** functions, namely binary search and exponential search. The latter
  is also known as galopping search.

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

## TODO

The utility-belt is still under development. The following is a list of things I
have yet to add.

### Re-exports for the prelude

- TODO A memoization lib

### Grids

- Grid2D class
  - TODO Ability to 'fold' grids like pieces of paper (along or, between two columns)

### Search for combinatorial optimization

- TODO: Anytime Beam Search
- TODO: Nested Search

#### Pathfinding

- TODO: Seidel's Algorithm

### Math

- TODO: Chinese Remainder Theorem
- TODO: Root finding (Newton?)
- TODO: Point-in-Polygon algorithm
- TODO: Shoelace formula (polygon area calculation)

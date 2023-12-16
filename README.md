# Utility-Belt

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

This is my Rust library of potentially useful tools for Advent of Code.

## Using

Add `utility_belt` to your `Cargo.toml`:

```toml
[dependencies]
utility_belt = { git = "https://github.com/DataWraith/utility-belt.git" }
```

In your Advent of Code projects, simply `use utility_belt::prelude::*;`.

## What's currently here?

### Re-exports

Most of the heavy-lifting is done by other libraries this crate re-exports:

- [ahash](https://docs.rs/ahash) - fast HashMap and HashSet implementation, hashing of single values
- [glam](https://docs.rs/glam) - convenient 2D vector types
- [indoc](https://docs.rs/indoc) - exposes macros for convenient inline String formatting
- [itertools](https://docs.rs/itertools) - various tools for working with iterators
- [ndarray](https://docs.rs/ndarray) - n-dimensional container for general elements
- [nom](https://docs.rs/nom) and [nom_locate](https://docs.rs/nom_locate/latest/nom_locate/) - ergonomic parser combinators
- [petgraph](https://docs.rs/petgraph) - a graph data structure library
- [pathfinding](https://docs.rs/pathfinding) - BFS, A*, Brent's algorithm for cycle finding, conncted components, etc.
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

- **Cumulative sum** helpers in 1D (`PrefixSum`) and 2D (`SummedAreaTable`).

  These allow you to quickly look up the sum of values in a given 1D range or 2D
  rectangle.

- **`nom` parsers** for `usize` and `isize` values.

  I plan to add more parsers over time.

- **search** functions, namely binary search and exponential search. The latter
  is also known as galopping search.

## TODO

The utility-belt is still under development. The following is a list of things I
have yet to add.

### Re-exports for the prelude

- TODO A memoization lib

### Grids

- Grid2D class
  - TODO Ability to 'fold' grids like pieces of paper (along or, between two columns)
  - TODO Auto-generate a palette when saving to PNG (Martin Ankerl method via fibonacci hashing)
  - TODO "Zoom in" by duplicating tiles
  - TODO "Zoom in" with provided templates (to replace certain tiles)

### Search for combinatorial optimization

- TODO: Branch&Bound
- TODO: Anytime Beam Search
- TODO: Nested Search, maybe Nested Rollout Policy Adaptation
- TODO: Nested Monte Carlo Search (low priority)

#### Pathfinding

- TODO: Seidel's Algorithm
- TODO: Anytime Focal Search (use a Radix heap)

### Math

- TODO: Chinese Remainder Theorem
- TODO: Root finding (Newton?)
- TODO: Ranges (Union, Intersection, One-sided intersection)

### Trees

- TODO: Maybe: Fenwick Tree
- TODO: SumTree / OrderStatistic Tree

### Graphics

- TODO: Digital Differential Analyzer

### Misc

- TODO: State-iteration helper

  The idea is to have a HashMap containing the current states. Then a transition
  function is applied to each state, and the resulting state(s) are collected in
  a new HashMap. The HashMap keeps track of how often a given state has
  occurred. This can be used to, for example, count how often a state is visited
  in a finite state machine after `n` iterations.

- TODO: Path contraction / memoization (not sure what to name this)

  Some Advent of Code puzzles involve finding the result of applying, say, one
  billion operations to a data structure. Since this kind of problem would be
  impossible otherwise, the problems usually contain a cycle we can find using,
  for example, the `pathfinding` crate and Brent's algorithm.

  Path contraction is an alternative that I want to investigate for this
  use-case. The idea is to make short-cuts in the state-space, similar to the
  `Contraction Hierarchies` idea in pathfinding.

  For example, given a path from A to E, `A -> B -> C -> D -> E`, we can start
  by moving from `A` to `B`, and then from `B` to `C`. Now that we know where
  the transitions lead, we can add a shortcut from `A` to `C`, skipping `B`.

  Short-cuts are themselves subject to being short-cut: When we're at `A` again,
  we move through the short-cut `A--->C`. If there is already a short-cut
  `C--->E`, we can combine the shortcuts to form a new shortcut `A--->E`.

  We store the length of each short-cut in order to be able to track how many
  operations we've done already. If a shortcut would overshoot the target, we
  can simply ignore it.

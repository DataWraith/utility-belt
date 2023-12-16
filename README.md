# Utility Belt

This is my library of useful tools for Advent of Code.

## TODO

(These are all marked with TODO so my TodoTree extension picks them up)

### Utility-belt prelude re-exports

- [x] ahash
- [x]: glam
- [x]: pathfinding
- [x]: ndarray
- [x]: nom, nom_locate
- [x]: itertools
- [x]: petgraph 
- [x]: rayon
- [x]: rstest (probably)
- TODO: A memoization lib
- [x] TinyVec
- [x] itertools
- [x] indoc

### Grids

- TODO: A Grid2D class
  - [x] Parsed from a chars() iter
  - [x]: Mirror Horizontally, Vertically
  - [x]: Rotate by +90/-90 degrees
  - TODO: Ability to save as PNG, possibly with auto-generated palette
  - TODO: "Zoom in" by duplicating tiles
  - TODO: "Zoom in" with provided templates (to replace certain tiles)
- TODO: A Wrapping Grid2D class (for toroidal problems)
- TODO: A Bordered Grid that implicitly includes a border around the input
- [x] Direction enum (NESW)
- [x]: DirectionSet (u8)
- [x]: Moore Neighborhood
- [x]: Von Neumann Neighborhood
- [x]: Knight's move neighborhood
- [x]: Adjacency testing

### Search

- [x] Binary search
- [x] Exponential Search
- TODO: Branch&Bound
- TODO: Anytime Beam Search
- TODO: Nested Search, maybe Nested Rollout Policy Adaptation
- TODO: Nested Monte Carlo Search (low priority)

#### Pathfinding

- TODO: Seidel's Algorithm
- TODO: Anytime Focal Search (use a Radix heap)

### Parsing

- Nom parsers that are frequently useful
  - `parse_usize`
  - `parse_isize`

### Math

- Greatest Common Divisor
- Least Common Multiple
- TODO: Chinese Remainder Theorem
- TODO: Root finding (Newton?)
- Prefix sum
- Summed Area Table
- TODO: Ranges (Union, Intersection, One-sided intersection)

### Trees

- TODO: Maybe: Fenwick Tree
- TODO: SumTree / OrderStatistic Tree

### Graphics

- TODO: Digital Differential Analyzer

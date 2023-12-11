# Utility Belt

This is my library of useful tools for Advent of Code.

## TODO

(These are all marked with TODO so my TodoTree extension picks them up)

### Grids

- TODO: A Grid2D class
  - TODO: Parsed from a chars() iter
  - TODO: Mirror Horizontally, Vertically
  - TODO: Rotate by +90/-90 degrees
  - TODO: Ability to save as PNG, possibly with auto-generated palette
  - TODO: "Zoom in" by duplicating tiles
  - TODO: "Zoom in" with provided templates (to replace certain tiles)
- TODO: A Wrapping Grid2D class (for toroidal problems)
- TODO: A Bordered Grid that implicitly includes a border around the input
- TODO: Direction enum (NESW)
- TODO: DirectionSet (u8)
- TODO: Moore Neighborhood
- TODO: Von Neumann Neighborhood
- TODO: Adjacency testing
- TODO: Coordinate struct (x/y coordinates)

### Search

- Binary search
- Exponential Search
- TODO: Anytime Beam Search
- TODO: Nested Search, maybe Nested Rollout Policy Adaptation
- TODO: Nested Monte Carlo Search (low priority)

#### Pathfinding

- Breadth-First Search
- TODO: Depth-First Search
- TODO: Uniform Cost Search (use a Radix heap)
- TODO: Seidel's Algorithm
- TODO: Anytime Focal Search (use a Radix heap)

### Parsing

- Nom parsers that are frequently useful
  - TODO: `parse_usize`
  - TODO: `parse_isize`

### Math

- Greatest Common Divisor
- Least Common Multiple
- TODO: Chinese Remainder Theorem
- TODO: Root finding (Newton?)
- TODO: Disjoint-Set datastructure
- Prefix sum
- Summed Area Table
- TODO: Ranges (Union, Intersection, One-sided intersection)

#### Graphs

- TODO: Cycle finding (Tortoise-Hare algorihtm)
- TODO: Hungarian Algorithm

### Trees

- TODO: Maybe: Fenwick Tree
- TODO: SumTree / OrderStatistic Tree

### Graphics

- TODO: Digital Differential Analyzer

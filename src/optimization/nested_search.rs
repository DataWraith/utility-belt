use std::hash::Hash;

/// Nested search algorithm
///
/// This is a generic implementation of the Nested Search algorithm. It takes a
/// level parameter that determines how deeply the search should nest, as well
/// as a start state and functions for generating successors, the heuristic move,
/// and for scoring states.
///
/// NOTE: Larger nesting levels take exponentially more time, so levels beyond 2
/// or 3 are usually not recommended)
pub fn nested_search<N, IN, C>(
    level: u8,
    start: &N,
    successors: &impl Fn(&N) -> IN,
    heuristic: &impl Fn(&N) -> Option<N>,
    score: &impl Fn(&N) -> C,
) -> N
where
    N: Eq + Clone + Hash,
    IN: IntoIterator<Item = N>,
    C: Ord + Clone,
{
    if level == 0 {
        return playout(start, heuristic);
    }

    let mut state = start.clone();
    let mut terminal = false;

    while !terminal {
        terminal = true;

        let mut best = None;
        let mut best_score = None;

        for s in successors(&state).into_iter() {
            terminal = false;

            let end_state = nested_search(level - 1, &s, successors, heuristic, score);
            let state_score = score(&end_state);

            if best.is_none() || state_score > best_score.clone().unwrap() {
                best = Some(s);
                best_score = Some(state_score);
            }
        }

        if !terminal {
            state = best.clone().unwrap();
        }
    }

    state
}

/// Playout function for nested search.
///
/// It chooses the first successor at each step. Since the successors are
/// ordered from best to worst, this is equivalent to choosing the best
/// successor.
fn playout<N, FN>(start: &N, heuristic: &FN) -> N
where
    N: Eq + Clone + Hash,
    FN: Fn(&N) -> Option<N>,
{
    let mut cur = start.clone();

    while let Some(next) = heuristic(&cur) {
        cur = next;
    }

    cur
}

#[cfg(test)]
mod tests {
    use ndarray::Array2;

    use super::*;

    // As a test case for nested search, we'll construct a small Latin square.
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct State {
        possibilities: Array2<Vec<usize>>,
    }

    impl State {
        pub fn new(dimensionality: usize) -> Self {
            let possible_values = (0..dimensionality).collect::<Vec<_>>();
            let possibilities =
                Array2::from_elem((dimensionality, dimensionality), possible_values);

            Self { possibilities }
        }

        pub fn assign(&self, x: usize, y: usize, value: usize) -> Self {
            assert!(self.possibilities[[y, x]].contains(&value));

            let mut result = self.clone();

            for ((py, px), square) in self.possibilities.indexed_iter() {
                if py != y && px != x {
                    continue;
                }

                result.possibilities[[py, px]] = square
                    .iter()
                    .filter(|v| **v != value)
                    .cloned()
                    .collect::<Vec<_>>();
            }

            result.possibilities[[y, x]] = vec![value];

            result
        }
    }

    #[test]
    fn test_latin_squares() {
        let dimensionality = 6;

        let successors = |state: &State| {
            let mut coordinates = Vec::new();

            for (y, row) in state.possibilities.outer_iter().enumerate() {
                for (x, square) in row.iter().enumerate() {
                    if square.is_empty() {
                        return vec![];
                    }

                    if square.len() > 1 {
                        coordinates.push((x, y));
                    }
                }
            }

            coordinates
                .into_iter()
                .flat_map(|(x, y)| {
                    state.possibilities[[y, x]]
                        .iter()
                        .map(move |v| state.assign(x, y, *v))
                })
                .collect()
        };

        let heuristic = |state: &State| {
            let mut coordinates = Vec::new();

            for (y, row) in state.possibilities.outer_iter().enumerate() {
                for (x, square) in row.iter().enumerate() {
                    if square.is_empty() {
                        return None;
                    }

                    if square.len() > 1 {
                        coordinates.push((x, y));
                    }
                }
            }

            coordinates
                .into_iter()
                .map(|(x, y)| (x, y, state.possibilities[[y, x]].len()))
                .min_by_key(|(_, _, len)| *len)
                .map(|(x, y, _)| (x, y))
                .map(|(x, y)| state.assign(x, y, state.possibilities[[y, x]][0]))
        };

        let score = |state: &State| {
            let mut score = 0isize;

            for row in state.possibilities.outer_iter() {
                let mut seen = vec![false; dimensionality];

                for square in row.iter() {
                    if square.is_empty() {
                        score -= 1;
                        continue;
                    }

                    let value = square[0];

                    if seen[value] {
                        score -= 1;
                        continue;
                    }

                    seen[value] = true;
                }
            }

            for col in state.possibilities.axis_iter(ndarray::Axis(1)) {
                let mut seen = vec![false; dimensionality];

                for square in col.iter() {
                    if square.is_empty() {
                        score -= 1;
                        continue;
                    }

                    let value = square[0];

                    if seen[value] {
                        score -= 1;
                        continue;
                    }

                    seen[value] = true;
                }
            }

            score
        };

        let start = State::new(dimensionality);
        let square = nested_search(1, &start, &successors, &heuristic, &score);

        assert_eq!(score(&square), 0);
    }
}

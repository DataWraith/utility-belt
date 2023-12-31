use std::hash::Hash;

use rand::prelude::*;

/// Nested Monte-Carlo Search algorithm
///
/// NOTE: Larger nesting levels take exponentially more time, so levels beyond 2
/// or 3 are usually not recommended.
pub fn nmcs<N, IN, C>(
    level: u8,
    start: &N,
    successors: &impl Fn(&N) -> IN,
    score: &impl Fn(&N) -> C,
    goal: &impl Fn(&N) -> bool,
) -> N
where
    N: Eq + Clone + Hash,
    IN: IntoIterator<Item = N>,
    C: Ord + Clone,
{
    let mut state = start.clone();
    let mut terminal = false;
    let mut overall_best = None;
    let mut overall_score = None;

    while !terminal {
        terminal = true;

        let mut cur_best = None;
        let mut cur_score = None;

        for next in successors(&state) {
            terminal = false;

            let sampled = if level <= 1 {
                playout(&next, successors)
            } else {
                nmcs(level - 1, &next, successors, score, goal)
            };

            let score = score(&sampled);

            if cur_best.is_none() || score > cur_score.clone().unwrap() {
                cur_best = Some(next.clone());
                cur_score = Some(score.clone());
            }

            if overall_best.is_none() || score > overall_score.clone().unwrap() {
                overall_best = Some(sampled.clone());
                overall_score = Some(score);

                if goal(&sampled) {
                    return sampled;
                }
            }
        }

        if !terminal {
            state = cur_best.clone().unwrap();
        }
    }

    overall_best.unwrap_or(start.clone())
}

fn playout<N, FN, IN>(start: &N, successors: &FN) -> N
where
    N: Eq + Clone + Hash,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut cur = start.clone();

    while let Some(next) = successors(&cur).into_iter().choose(&mut rand::thread_rng()) {
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
        let dimensionality = 7;

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
                .map(|(x, y)| state.assign(x, y, state.possibilities[[y, x]][0]))
                .collect()
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
        let square = nmcs(2, &start, &successors, &score, &|s: &State| score(s) == 0);

        assert_eq!(score(&square), 0);
    }
}

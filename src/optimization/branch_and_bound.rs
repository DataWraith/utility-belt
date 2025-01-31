/// Branch and bound
///
/// This is a generic implementation of the Branch and bound-algorithm. It is
/// useful for finding the best solution to a problem where the solution space
/// is too large to search exhaustively.
///
/// The idea is to keep track of a lower bound on the cost of the best solution.
/// If a candidate is considered whose lower bound on the cost is higher than
/// the current best solution, the candidate is discarded, because it cannot
/// possibly be better than the current best solution.
///
/// This version minimizes the cost of the solution.
///
/// # Arguments
///
/// * `start` - The starting state
/// * `successors` - A function that returns the successors of a given state (e.g. as a Vec).
/// * `cost` - A function that returns the cost of a full solution (e.g. the length of a path).
///    NOTE: If the state is not a solution or final state this MUST return None.
/// * `bound` - A function that returns a lower bound on the cost of the given solution.
///
pub fn branch_and_bound<N, FN, FC, FB, IN, C>(
    start: &N,
    mut successors: FN,
    mut cost: FC,
    mut bound: FB,
) -> N
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    FC: FnMut(&N) -> Option<C>,
    FB: FnMut(&N) -> C,
    IN: IntoIterator<Item = N>,
    C: Ord + Copy,
{
    let mut stack = vec![start.clone()];
    let mut best = None;
    let mut best_n = start.clone();

    while let Some(cur) = stack.pop() {
        if let Some(cost) = cost(&cur) {
            if best.is_none() || cost < best.unwrap() {
                best = Some(cost);
                best_n = cur.clone();
            }

            continue;
        }

        for next in successors(&cur) {
            if best.is_none() || bound(&next) < best.unwrap() {
                stack.push(next);
            }
        }
    }

    best_n
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::prelude::*;

    #[test]
    fn test_branch_and_bound() {
        let start = 0;

        let mut seen = Set::new(1u8);

        let successors = |n: &i32| {
            let mut result = Vec::new();

            if *n < 5 {
                if !seen.test_bit((n + 1) as u32) {
                    seen.set_bit((n + 1) as u32);
                    result.push(n + 1);
                }

                if *n != 0 && !seen.test_bit((n - 1) as u32) {
                    seen.set_bit((n - 1) as u32);
                    result.push(n - 1);
                }
            }

            result
        };

        let cost = |n: &i32| {
            if *n == 5 {
                Some(0)
            } else {
                None
            }
        };

        let bound = |n: &i32| 5 - n;

        assert_eq!(branch_and_bound(&start, successors, cost, bound), 5);
    }
}

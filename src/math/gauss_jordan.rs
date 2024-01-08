use ndarray::{Array1, Array2};

pub const EPSILON: f64 = 1e-9;

#[derive(Debug, PartialEq)]
pub enum Solution {
    Unique(Array1<f64>),
    Infinite(Array1<f64>),
    None,
}

/// Perform Gauss-Jordan elimination on a matrix.
///
/// The system of linear algebraic equations needs to be represented as a matrix;
/// the last column of the matrix is taken to be the right-hand side of the equations
/// (often denoted as the vector `b`)
///
/// Reference: https://cp-algorithms.com/linear_algebra/linear-system-gauss.html
pub fn gauss_jordan(mut matrix: Array2<f64>) -> Solution {
    let (n, m) = matrix.dim();
    let m = m - 1;

    let mut nonzero = vec![None; m];

    let mut col = 0;
    let mut row = 0;

    while col < m && row < n {
        let mut sel = row;

        // Select pivot
        for i in row..n {
            if matrix[[i, col]].abs() > matrix[[sel, col]].abs() {
                sel = i;
            }
        }

        if matrix[[sel, col]].abs() < EPSILON {
            // No pivot found in this column, try the next one
            col += 1;
            continue;
        }

        // Swap current row with pivot
        for i in col..=m {
            let tmp = matrix[[sel, i]];
            matrix[[sel, i]] = matrix[[row, i]];
            matrix[[row, i]] = tmp;
        }

        nonzero[col] = Some(row);

        for i in 0..n {
            if i == row {
                continue;
            }

            let c = matrix[[i, col]] / matrix[[row, col]];

            for j in col..=m {
                matrix[[i, j]] -= matrix[[row, j]] * c;
            }
        }

        col += 1;
        row += 1;
    }

    let mut ans = Array1::zeros(m);

    for i in 0..m {
        if let Some(nz) = nonzero[i] {
            ans[i] = matrix[[nz, m]] / matrix[[nz, i]];
        }
    }

    for i in 0..n {
        let mut sum = 0f64;

        for j in 0..m {
            sum += ans[j] * matrix[[i, j]];
        }

        if (sum - matrix[[i, m]]).abs() > EPSILON {
            return Solution::None;
        }
    }

    if nonzero.iter().any(Option::is_none) {
        return Solution::Infinite(ans);
    }

    Solution::Unique(ans)
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_line_intersection_case1() {
        // http://homepages.math.uic.edu/~rmlowman/math160/160s10W1L2-gaussian.pdf
        let matrix = array![[2.0, 3.0, 8.0], [6.0, -2.0, 2.0]];
        let expected = array![1.0, 2.0];

        let ans = gauss_jordan(matrix);

        if let Solution::Unique(ans) = ans {
            for i in 0..2 {
                assert!(
                    (ans[i] - expected[i]).abs() < EPSILON,
                    "Expected {} but got {}",
                    expected[i],
                    ans[i]
                )
            }
        } else {
            panic!("Expected unique solution");
        }
    }

    #[test]
    fn test_line_intersection_case2() {
        // http://homepages.math.uic.edu/~rmlowman/math160/160s10W1L2-gaussian.pdf

        let matrix = array![[2.0, -1.0, -2.0], [-2.0, 1.0, 1.0]];
        let ans = gauss_jordan(matrix);

        assert_eq!(ans, Solution::None);
    }

    #[test]
    fn test_line_intersection_case3() {
        // http://homepages.math.uic.edu/~rmlowman/math160/160s10W1L2-gaussian.pdf

        let matrix = array![[2.0, -1.0, -1.0], [4.0, -2.0, -2.0]];
        let ans = gauss_jordan(matrix);

        if let Solution::Infinite(_) = ans {
            // Infinite number of solutions found
        } else {
            panic!("Expected infinite solutions");
        }
    }

    #[test]
    fn test_wikipedia_example() {
        // https://en.wikipedia.org/wiki/Gaussian_elimination#Example_of_the_algorithm
        let matrix = array![
            [2.0, 1.0, -1.0, 8.0],
            [-3.0, -1.0, 2.0, -11.0],
            [-2.0, 1.0, 2.0, -3.0]
        ];

        let expected = array![2.0, 3.0, -1.0];

        let ans = gauss_jordan(matrix);

        if let Solution::Unique(ans) = ans {
            for i in 0..3 {
                assert!(
                    (ans[i] - expected[i]).abs() < EPSILON,
                    "Expected {} but got {}",
                    expected[i],
                    ans[i]
                )
            }
        } else {
            panic!("Expected unique solution");
        }
    }
}

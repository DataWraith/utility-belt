use ndarray::{Array1, Array2};
use num::{Num, Signed};

#[derive(Debug, PartialEq)]
pub enum Solution {
    Unique,
    Infinite,
    None,
}

/// Perform Gauss-Jordan elimination on a matrix.
///
/// If you want to solve a system of linear algebraic equations, the system
/// needs to be represented as a matrix; the last column of the matrix is taken
/// to be the right-hand side of the equations (often denoted as the vector `b`).
///
/// Reference: https://cp-algorithms.com/linear_algebra/linear-system-gauss.html
///
/// # Arguments
///
/// * `matrix` - The matrix representing the system of linear equations.
/// * `ans` - The array to store the answer in.
/// * `eps` - A small value (e.g. 1e-9) to help with floating point precision.
///
/// # Returns
///
/// * `Solution::Unique` if the system has a unique solution.
/// * `Solution::Infinite` if the system has infinitely many solutions.
/// * `Solution::None` if the system has no solutions.
///
/// # Panics
///
/// * If the number of unknown variables does not match the length of the answer array.
pub fn gauss_jordan<T: Num + Signed + PartialOrd + Clone>(
    mut matrix: Array2<T>,
    ans: &mut Array1<T>,
    eps: T,
) -> Solution {
    let (n, m) = matrix.dim();

    // Account for the last column being the right-hand side of the equations
    let m = m - 1;

    assert_eq!(
        m,
        ans.len(),
        "Number of unknown variables does not match length of answer array"
    );

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

        if matrix[[sel, col]].abs() <= eps {
            // No pivot found in this column, try the next one
            col += 1;
            continue;
        }

        // Swap current row with pivot
        for i in col..=m {
            let tmp = matrix[[sel, i]].clone();
            matrix[[sel, i]] = matrix[[row, i]].clone();
            matrix[[row, i]] = tmp;
        }

        nonzero[col] = Some(row);

        for i in 0..n {
            if i == row {
                continue;
            }

            let c = matrix[[i, col]].clone() / matrix[[row, col]].clone();

            for j in col..=m {
                matrix[[i, j]] = matrix[[i, j]].clone() - matrix[[row, j]].clone() * c.clone();
            }
        }

        col += 1;
        row += 1;
    }

    for i in 0..m {
        if let Some(nz) = nonzero[i] {
            ans[i] = matrix[[nz, m]].clone() / matrix[[nz, i]].clone();
        }
    }

    for i in 0..n {
        let mut sum = T::zero();

        for j in 0..m {
            sum = sum + ans[j].clone() * matrix[[i, j]].clone();
        }

        if (sum - matrix[[i, m]].clone()).abs() > eps {
            return Solution::None;
        }
    }

    if nonzero.iter().any(Option::is_none) {
        return Solution::Infinite;
    }

    Solution::Unique
}

#[cfg(test)]
mod tests {
    use super::*;

    use ndarray::array;

    #[test]
    fn test_line_intersection_case1() {
        // http://homepages.math.uic.edu/~rmlowman/math160/160s10W1L2-gaussian.pdf
        let matrix = array![[2.0, 3.0, 8.0], [6.0, -2.0, 2.0]];
        let expected = array![1.0, 2.0];
        let eps = 1e-9;

        let mut ans = array![0.0, 0.0];

        let soln = gauss_jordan(matrix, &mut ans, eps);

        assert_eq!(soln, Solution::Unique);

        for i in 0..2 {
            assert!(
                (ans[i] - expected[i]).abs() < eps,
                "Expected {} but got {}",
                expected[i],
                ans[i]
            )
        }
    }

    #[test]
    fn test_line_intersection_case2() {
        // http://homepages.math.uic.edu/~rmlowman/math160/160s10W1L2-gaussian.pdf

        let matrix = array![[2.0, -1.0, -2.0], [-2.0, 1.0, 1.0]];
        let mut ans = array![0.0, 0.0];

        let eps = 1e-9;
        let soln = gauss_jordan(matrix, &mut ans, eps);

        assert_eq!(soln, Solution::None);
    }

    #[test]
    fn test_line_intersection_case3() {
        // http://homepages.math.uic.edu/~rmlowman/math160/160s10W1L2-gaussian.pdf

        let matrix = array![[2.0, -1.0, -1.0], [4.0, -2.0, -2.0]];
        let mut ans = array![0.0, 0.0];

        let eps = 1e-9;
        let soln = gauss_jordan(matrix, &mut ans, eps);

        assert_eq!(soln, Solution::Infinite);
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

        let mut ans = array![0.0, 0.0, 0.0];

        let eps = 1e-9;
        let soln = gauss_jordan(matrix, &mut ans, eps);

        assert_eq!(soln, Solution::Unique);

        for i in 0..3 {
            assert!(
                (ans[i] - expected[i]).abs() < eps,
                "Expected {} but got {}",
                expected[i],
                ans[i]
            )
        }
    }

    #[test]
    fn test_big_rational() {
        let zero = num::BigRational::from_integer(0.into());
        let one = num::BigRational::from_integer(1.into());
        let two = num::BigRational::from_integer(2.into());
        let three = num::BigRational::from_integer(3.into());
        let eight = num::BigRational::from_integer(8.into());
        let eleven = num::BigRational::from_integer(11.into());

        let matrix = array![
            [two.clone(), one.clone(), -one.clone(), eight.clone()],
            [-three.clone(), -one.clone(), two.clone(), -eleven.clone()],
            [-two.clone(), one.clone(), two.clone(), -three.clone()]
        ];

        let expected = array![two.clone(), three.clone(), -one.clone()];

        let mut ans = array![zero.clone(), zero.clone(), zero.clone()];

        let eps = zero;
        let soln = gauss_jordan(matrix, &mut ans, eps);

        assert_eq!(soln, Solution::Unique);

        for i in 0..3 {
            assert_eq!(ans[i], expected[i]);
        }
    }
}
